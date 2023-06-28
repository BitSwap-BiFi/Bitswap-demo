# BitSwap (Core) ⚡

Exchange for RGB assets

**Disclaimer**: Alpha Software

### How Works?

The user can participate of Pools and earn BTC or USDT like on Uniswap

How there's no lunch free, you can lose sats in these scenarios:

- Illiquid asset

- Stop Loss

- Channel Liquidity on Lightning Network

- DLC fail

### Swaps

- Alice create invoice (RGB or LN) for Bob receive USDT

- Bob receive USDT tokens of Alice

- Alice receive BTC of Bob

### Roadmap

- [X] Testnet
- [x] Integration with LDK
- [ ] Mainnet
- [x] Fees
- [ ] Beta app
- [ ] UI/UX
- [ ] Stop Loss
- [ ] Payjoin
- [ ] Official USDT implementation
- [X] BTC/USDT pair
- [x] Open source code

### Run BitSwap

You can add in your cargo.toml

```cargo.toml
[package]
rgb-core = "0.10.4"
rust-dlc = "0.4"
bitswap = "1.0.4"
aluvm= "0.10.3"
rgb-lighting-sample = "0.1.0"

