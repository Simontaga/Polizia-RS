FROM rust:1.65 as builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update
RUN apt-get install -y libssl-dev
RUN apt-get install -y libmariadb-dev-compat
RUN apt-get install -y libmariadb-dev
RUN apt-get install -y default-libmysqlclient-dev

RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/polizia_rs ./target/release/polizia_rs
CMD ["./target/release/polizia_rs"]