ARG BUILD_IMAGE=rust:1.50-buster@sha256:25a32551f42722169ea0b50b9ea752bd43a31b3e52371e735ea675866e4eb164
ARG RUN_IMAGE=debian:buster-slim@sha256:8bf6c883f182cfed6375bd21dbf3686d4276a2f4c11edc28f53bd3f6be657c94

FROM $BUILD_IMAGE as builder
ARG TARGET=server

WORKDIR /usr/src/rust-graphql-sonic
COPY . .

ENV TARGET=${TARGET}
RUN cargo install --bin $TARGET --path .


FROM $RUN_IMAGE
ARG TARGET=server

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

ENV TARGET=${TARGET}
COPY --from=builder /usr/local/cargo/bin/$TARGET /usr/local/bin/rust-graphql-sonic

CMD ["rust-graphql-sonic"]
