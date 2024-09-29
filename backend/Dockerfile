FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install --path .

RUN cargo build --release

EXPOSE 8000

CMD ["shelf-watcher-backend"]