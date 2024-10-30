use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use anyhow::Ok;
use axum::{
    body::Bytes,
    extract::{MatchedPath, Request},
    http::HeaderMap,
    response::Response,
};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::layer::SubscriberExt;

use crate::{config::CONFIG, state::AppState};

pub struct Server;

impl Server {
    pub async fn run() -> anyhow::Result<()> {
        tracing_subscriber::registry().with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "mobile_vault_server_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        );
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);
        let mut database_connection_options =
            ConnectOptions::new(&CONFIG.database_connection_string);

        database_connection_options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(database_connection_options).await?;
        let app_state = AppState::from(&db);
        let app = crate::routes::router().with_state(app_state).layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );
        let listener = TcpListener::bind(addr).await?;

        tracing::debug!("server running on http://{addr}");
        axum::serve(listener, app).await?;

        Ok(())
    }
}
