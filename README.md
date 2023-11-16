# Bitswap (Core) 🟠 ⚡ 💱

[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)


*Decentralized Exchange for RGB assets*

⚠️**Alpha Software and WIP**

⚠️**Unstable Software**

⚠️**Isn't ready to compile via Cargo.lock for while**


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

- DLCs verify price settled and solve 2-of-2 pairs

### Features

- Payjoin
- Universal swaps, atomic swaps via on-chain and Lightning Network
-  P2P Swaps without intermediary with DLCs
- Taproot
- LDK and RGB Lightning Node
- Prime
- Bifrost
- CLI
- BOLT12
- Non custodial via on-chain and Lightning Network
- Privacy
- Non KYC

## Roadmap

- [ ] Mainnet
- [ ] Beta app
- [ ] UI/UX
- [ ] Web server and domain
- [x] CLI 
- [x] On-Chain RGB Assets with Swap using [Bitcoin Protocol](https://github.com/BP-WG/bp-core)
- [ ] User friendly UI for allow mint tokens, send, receive and swap tokens on Lightning Network and DEX
- [ ] Taproot Channels by LDK
- [x] RGB wallet (new version stable) [WIP]
- [x] Payjoin and Taproot full implementation (WIP)
- [ ] Official USDT implementation by Bitifinex
- [x] Integration with [COSM wallet](http://www.cosminmart.com/#/Wallet) [WIP]
- [x] Bifrost (WIP)
- [x] Prime (WIP)
- [ ] Integration with APIs from [RGBex](https://rgbex.io/)
- [x] RGB Node (WIP)
- [x] Complete PRs pendents (WIP)
- [ ] [BitMask](https://bitmask.app/)
- [x] RGB Proxy (WIP)
- [ ] Add [RGB CLI](https://github.com/RGB-WG/rgb) as tool complementary for CLI from DEX

## Run BitSwap

### Add to the your Cargo

You can add in your cargo.toml

```cargo.toml
[package]
rgb-core = "0.11.0"
rust-dlc = "0.4"
bitswap = "1.0.9"
rust-aluvm = "0.10.5"
rust-baid58 = "0.4.4"
rust-lightning = "0.0.118"
bp-core = "0.11.0"
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
-opeen_channel
-close_channel

```

## Documentation for run DEX 

[Soon](https://github.com/BitSwap-BiFi/Bitswap-core/tree/main/doc)


## Documentation about DEX

[Official Documentation](https://github.com/BitSwap-BiFi/bitswap-docs)

## License ⚠️

The code is licensed under either:

-  [Business Source License 1.1](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/LICENSE.md)
-  This licensed under [BSL 1.1](https://mariadb.com/bsl11/), a "source available" license which automatically turns into an open source license after 4 years, see [LICENSE](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/LICENSE.md) and [BSL FAQs](https://mariadb.com/bsl-faq-mariadb/) for details. 


## Ossification DEX

In this Core, we'll not integrate with Sidechains non built on RGB and altcoins beyond RGB, Bitcoin, Prime, Bifrost, RGB wallets and Lightning Network.

## FAQ

Check [here](https://github.com/BitSwap-BiFi/Bitswap-FAQ/)
