FROM rust:1.84.1

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

ENV RUST_LOG=info

CMD ["./target/release/battle-simulator"]