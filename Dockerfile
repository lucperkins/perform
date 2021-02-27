FROM rust AS builder

WORKDIR /build

COPY . ./

RUN cargo build --bin perform

FROM debian:buster-slim

COPY --from=builder /build/target/debug/perform /usr/local/bin

ENTRYPOINT ["/usr/local/bin/perform"]