ARG RUST_VERSION=1.82.0
ARG APP_NAME=vibranium

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME

# crea
WORKDIR /sorce_dir 

RUN mkdir app

COPY Cargo.toml ./app
COPY ../proto .
COPY src ./app
COPY ../database .
COPY build.rs ./app 

RUN apk add --no-cache clang lld musl-dev git

RUN cargo build  --release

RUN cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 50051

# What the container should run when it is started.
CMD ["/bin/server"]
