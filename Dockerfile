FROM rust:1.61.0

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/DOS"]