FROM rust:1-alpine AS build
RUN apk add build-base
WORKDIR "src"
COPY . .
WORKDIR "cube_shuffle-cli"
RUN cargo install --path .

FROM alpine:3 AS run
COPY --from=build /usr/local/cargo/bin/cube_shuffle-cli /usr/local/bin/cube_shuffle
# Verify basic operation
RUN cube_shuffle -V
ENTRYPOINT ["cube_shuffle"]