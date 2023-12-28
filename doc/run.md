# Run Bitswap

Welcome to the Documentation around Bitswap

**DEX running on Testnet**
## Pre Requisite

- RGB CLI
- RGB Core
- RGB STD
- Rust
- Rust Lightning
- Bitcoin Core Node
- Electrum Node
- Lightning Node (testnet)
- Sats for fund wallet allow send, receive and swap RGB Assets

### Clone the Git repository

```git
git clone https://github.com/Bitswap-BiFi/Bitswap-core.git
```

### Navigate to the project directory
```cd
cd Bitswapcore
```

### Build project to generate/update Cargo.lock
```cargo
cargo build
```

### Run

```cargo
cargo run
```

## Use CLI

Use ``bit`` for see options and run DEX
```cli
$ bit--help
A CLI for manage channels, swaps and liquidity around RGB and Lightning Network

[arguments]

$bit - for bitswap management

-contract
-wallet
-swap
-opeen_channel
-close_channel

```
