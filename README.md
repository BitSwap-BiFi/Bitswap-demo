# BitSwap (Core) ⚡

Exchange for RGB assets

### How Works?

-Create token using UTXO

-Sell token

-Buy token

We'll use RGB-20 as base first.

### Feature

We'll use AMM (Automed Market Maker) on Exchange

-Alice create invoice for B

-Bob receive of Alice tokens 

-Alice receive new tokens of swap

We'll use the model as like on [OBD](https://github.com/omnilaboratory/OmniBOLT-spec/blob/master/OmniBOLT-06-Automatic-Market-Maker-and-DEX.md)

### Roadmap

- [X] Testnet
- [ ] Integration with RGB Node/Bitguard/DLCs/LDK
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

