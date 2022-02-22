FROM rust:1-alpine AS build
RUN apk add build-base
WORKDIR "src"
COPY "cube_shuffle-core/" "cube_shuffle-core/"
COPY "cube_shuffle-cli/" "cube_shuffle-cli/"
WORKDIR "cube_shuffle-cli"
COPY "Cargo.lock" .
RUN cargo install --locked --path .

FROM alpine:3 AS run
COPY --from=build /usr/local/cargo/bin/cube_shuffle-cli /usr/local/bin/cube_shuffle
# Verify basic operation
RUN cube_shuffle -V
ENTRYPOINT ["cube_shuffle"]