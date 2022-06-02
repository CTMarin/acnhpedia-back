FROM rust:buster AS base

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY .. /code

EXPOSE 5000

FROM base AS builder

RUN cargo build --release

EXPOSE 5000

FROM debian:buster-slim

EXPOSE 5000

COPY --from=builder /code/target/release/acnhpedia-back /acnhpedia-back

CMD [ "/acnhpedia-back" ]