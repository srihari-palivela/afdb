# Build and run AFDB API server
FROM rust:1.79 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --example api_server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/examples/api_server /usr/local/bin/afdb-api
EXPOSE 8090
ENV RUST_LOG=info
CMD ["/usr/local/bin/afdb-api"]
