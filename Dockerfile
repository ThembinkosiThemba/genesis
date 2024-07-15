FROM rust:1.67 as builder
WORKDIR /usr/src/genesis
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/genesis /usr/local/bin/genesis
CMD ["genesis"]

# docker build -t genesis
# docker run -it --rm --name 