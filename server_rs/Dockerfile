# syntax=docker/dockerfile:1

FROM rust:1.81

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8198

CMD ["./target/release/server_rs"]
