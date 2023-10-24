# Bitswap (Core) ⚡ 💱

*Decentralized Exchange for RGB assets*

⚠️**Disclaimer 1**: Alpha Software

⚠️**Disclaimer 2**: Unstable Software

⚠️**Disclaimer 3**: Isn't ready to compile via Cargo.lock for while

**Contributions are welcome**

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
- Universal swaps, atomic swaps via on-chain and Lightning Network
- Taproot
- LDK and RGB Lightning Node

## Roadmap

- [ ] Mainnet
- [ ] Beta app
- [ ] UI/UX
- [ ] Web server and domain
- [x] CLI (WIP)
- [x] On-Chain RGB Assets with Swap using [Bitcoin Protocol](https://github.com/BP-WG/bp-core)
- [ ] User friendly UI for allow mint tokens, send, receive and swap tokens on Lightning Network and DEX
- [ ] Taproot Channels by LDK
- [x] RGB wallet (new version stable) [WIP]
- [x] Payjoin and Taproot full implementation (WIP)
- [ ] Official USDT implementation by Bitifinex
- [ ] Integration with BitMask and COSM wallet (APIs)
- [ ] Bifrost
- [ ] Integration with APIs from [RGBex](https://rgbex.io/)

## Run BitSwap

### Add to the your Cargo

You can add in your cargo.toml

```cargo.toml
[package]
rgb-core = "0.10.8"
rust-dlc = "0.4"
bitswap = "1.0.9"
rust-aluvm = "0.10.5"
rust-baid58 = "0.4.4"
rust-lightning = "0.0.117"
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

### Documentation

[Soon](https://github.com/BitSwap-BiFi/Bitswap-core/tree/main/doc)

## License

The code is licensed under either:

-  [Business Source License 1.1](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/LICENSE.md)


## Ossification DEX

In this Core, we'll not integrate with Sidechains or shitcoins (altcoins) beyond RGB, Bitcoin and Lightning Network.

## FAQ

Check [here](https://github.com/BitSwap-BiFi/Bitswap-FAQ/)
