FROM rust:1.75 as builder

WORKDIR /usr/src/hurl

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --bin hurl

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/hurl/target/release/hurl /usr/local/bin/hurl

COPY completions/hurl.bash /etc/bash_completion.d/hurl
COPY man/hurl.1 /usr/local/share/man/man1/hurl.1

RUN chmod +x /usr/local/bin/hurl

WORKDIR /workspace

VOLUME ["/workspace", "/home"]

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD hurl --version > /dev/null 2>&1 || exit 1

ENTRYPOINT ["hurl"]
CMD ["--help"]
