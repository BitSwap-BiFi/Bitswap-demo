FROM rust:1.79.0 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y git mingw-w64

RUN rustup target add x86_64-pc-windows-gnu

RUN git clone https://github.com/Bitswap-BiFi-Bitswap-demo.git /app/Bitswap-BiFi-Bitswap-demo

WORKDIR /app/Bitswap-BiFi-Bitswap-demo

RUN cargo build --release --target x86_64-pc-windows-gnu

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/Bitswap-BiFi-Bitswap-demo/target/x86_64-pc-windows-gnu/release/main.exe .

CMD ["/.bin.windows"./main.exe", "main.rs", "command.rs"]
RUN cd Bitswap-BiFi-Bitswap-demo && cargo build 
RUN --releaseCOPY --from=builder /Bitswap-BiFi-Bitswap-demo/target/release/main.
RUN --releaseCMD ["./main"]
CMD ["main.rs", "command.rs"]
