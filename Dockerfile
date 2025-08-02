FROM rust:1.88-alpine3.22 AS build

RUN apk add musl-dev
#RUN cargo install wol-relay --features cli

RUN --mount=type=bind,source=.,target=/code \
    cargo build \
        --manifest-path=/code/Cargo.toml \
        --target-dir /target \
        --release \
        --features cli

FROM alpine:3.22
COPY --from=build /target/release/wol-relay /usr/local/bin/wol-relay

ENTRYPOINT [ "/usr/local/bin/wol-relay" ]
