FROM rust:1.40 as builder

WORKDIR /build

COPY . .

RUN cargo build --release --frozen --locked

FROM debian:buster-slim

COPY --from=builder /build/release/rust-todo-api /usr/local/bin/rust-todo-api

CMD [ "rust-todo-api" ]
