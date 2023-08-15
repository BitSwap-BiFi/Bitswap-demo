# BitSwap (Core) ⚡ 💱

Exchange for RGB assets

⚠️**Disclaimer 1**: Alpha Software

⚠️**Disclaimer 2**: Unstable Software

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

### Features

- Payjoin
- Swaps between BTC/USDT
- Taproot
- LDK/RGB Lightning Node/LNP

## Demo and PoC

We create our demo around DEX

[Demo video](https://github.com/BitSwap-BiFi/Bitswap-PoC/)

## Roadmap

- [ ] Add new pairs for dCHF, RGBTC and RGBEX tokens
- [ ] Mainnet
- [ ] Beta app
- [ ] UI/UX
- [x] CLI
- [ ] On-Chain RGB Assets with Swap
- [ ] LNP
- [ ] RGB Lightning Network
- [ ] Payjoin full implementation
- [ ] Official USDT implementation
- [ ] Integration with APIs from [RGBex](https://rgbex.io/)

## Run BitSwap

### Add to the your Cargo

You can add in your cargo.toml

```cargo.toml
[package]
rgb-core = "0.10.7"
rust-dlc = "0.4"
bitswap = "1.0.8"
rust-aluvm = "0.10.5"
rust-baid58 = "0.4.4"
rust-lightning = "0.0.116"
payjoin = { version = {"0.9.0"}, features = ["send", "receive"] }
```
### Via CLI Command

```cli
$ bit--help
A CLI for manage channels, swaps and liquidity around RGB and Lightning Network

[arguments]

$bit - for bitswap management

-contract
-wallet
-swap
-mint
-let_channel
-close_channel

```
## License

Licensed under either:

-  Apache License, Version 2.0 
-  MIT License
-  GNU General Public License v3.0

## FAQ

Check [here](https://github.com/BitSwap-BiFi/Bitswap-FAQ/)
