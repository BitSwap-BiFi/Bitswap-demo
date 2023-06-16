# BitSwap (Core) ⚡

Exchange for RGB assets

### How Works?

The user can create a assets based on UTXO and after buy/sell this asset

### Feature

We use AMM (Automed Market Maker)

- Alice create invoice for B

- Bob receive of Alice tokens 

- Alice receive new tokens of swap

### Roadmap

- [X] Testnet
- [ ] Integration with LDK
- [ ] Mainnet
- [x] Fees
- [X] Using Contractum
- [X] BTC/USDT pair
- [x] Open source code

### Run BitSwap

You can in your cargo.toml

```cargo.toml
[package]
rgb-core = "0.10.3"
rust-dlc = "0.4"
bitswap = "1.0.3"
rgb-lighting-sample= "0.1.0"

