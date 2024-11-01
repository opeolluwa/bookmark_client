use crate::proto::activity_log::{activity_log_server::ActivityLog, LogActivityRequest, LogActivityResponse};

#[derive(Default, Clone)]
pub struct ActivityLogImplementation {}

#[tonic::async_trait]
impl ActivityLog for ActivityLogImplementation {
    #[must_use]
    #[allow(
        // elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn log_activity<'life0, 'async_trait>(
        &'life0 self,
        _request: tonic::Request<LogActivityRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<LogActivityResponse>,
                        tonic::Status,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
