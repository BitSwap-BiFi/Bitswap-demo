FROM rust:1.79.0 as builder
WORKDIR /app
RUN apt-get update && apt-get install -y \ git \
RUN git clone https://github.com/Bitswap-BiFi-Bitswap-demo.git /path/to/clone
WORKDIR /app/Bitswap-BiFi-Bitswap-demo
RUN cd Bitswap-BiFi-Bitswap-demo && cargo build 
RUN --releasCOPY --from=builder /Bitswap-BiFi-Bitswap-demo/target/release/main.
RUN --releaseCMD ["./main"]
CMD ["main.rs", "command.rs"]
