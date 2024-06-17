# Run Bitswap 💱

Welcome to the Documentation around Bitswap ⚡

⚠️**DEX running on Testnet, Signet & Regtest**

⚠️**Use small amount**

### Pre Requisite

- RGB CLI
- Rust
- Rust Lightning
- Rust DLC
- Sats for fund wallet allow send, receive and swap RGB Assets (Signet, Testnet)
- Use Bitlight demo contract RGB20, Bihelix-CLI, RGB/STD Interface, RGB CLI, Bitlight asset registries or Bitmask asset registries for create your asset and test into Demo DEX, which support last update

### Clone the Git repository

```git
git clone https://github.com/Bitswap-BiFi/Bitswap-demo.git
```

### Navigate to the project directory
```cd
cd Bitswapdemo
```

### Build project to generate/update Cargo.lock
```cargo
cargo build
cargo build --release
```

### Run

```cargo
cargo run
```
## Rustup update

```rustup
rustup update
```
## Use CLI

Use ``app`` for see options and run DEX
```cli
$ bit--help
A CLI for manage channels, swaps and liquidity around RGB and Lightning Network

[arguments]

$app - for bitswap management

-contract
-wallet
-swap
-opeen_channel
-close_channel

```
## Run via Docker

TBD
