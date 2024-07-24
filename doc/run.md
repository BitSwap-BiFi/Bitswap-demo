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


### Cargo install packages

```shell
curl -sSf https://sh.rustup.rs | sh

cargo install rgbstd
cargo install rgb
cargo install rgbcore
cargo install bpstd
cargo install rust
cargo install rustdlc
cargo install rustlightning
cargo install rustbitcoin
cargo install bdk
cargo install bdk-cli
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

## Generate private keys using BDK

```rust
bdk-cli generate-keys
```
## Deposit bitcoin via signet or testnet
```rust
appcli  sendtoaddress <address> <amount>
```
## Create a wallet
```rust
appcli createwallet <wallet_name>
```
**Use BDK**
```rust
bdk-cli create-wallet <wallet_name>
```
## Create a contract
```rust
appcli createcontract <contract_name> <wallet_name> <amount> <asset_name>
```
## Swap
```rust
appcli swap <wallet_name> <amount> <asset_name> <contract_name>
```
## Run via Docker

```docker
docker run -it --rm -v $(pwd):/app -w /app rust:latest bash
```

## Run via Docker Compose

```docker
docker-compose up
```

## Run via Docker Compose with RGB CLI

```docker
docker-compose -f docker-compose-cli.yml up
```
