FROM rust:1.85 as builder
WORKDIR /usr/src/rootftp

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates curl \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m rootftp

WORKDIR /home/rootftp

COPY --from=builder /usr/src/rootftp/target/release/rootftp .

RUN chown -R rootftp:rootftp .

USER rootftp

EXPOSE 2121

CMD ["./rootftp"]
