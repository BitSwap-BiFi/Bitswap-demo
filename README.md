# BitSwap (Core) ⚡

Exchange for RGB assets

### How Works?

The user can create a assets based on UTXO and after buy/sell this asset

### Feature

We'll use AMM (Automed Market Maker) on Exchange

-Alice create invoice for B

-Bob receive of Alice tokens 

-Alice receive new tokens of swap

We use the model as on [OBD](https://github.com/omnilaboratory/OmniBOLT-spec/blob/master/OmniBOLT-06-Automatic-Market-Maker-and-DEX.md)

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
rgb-core = "0.10"
rust-dlc = "0.4"
bitswap = "1.0.3"
rgb-lighting-sample= "0.1.0"

