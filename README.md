# Bitswap Demo ⚡ 💱

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Crates](https://img.shields.io/badge/crates-passing-brightgreen)
![Bitcoin Passing](https://img.shields.io/badge/bitcoin-passing-orange)
![Lightning Passing](https://img.shields.io/badge/lightning-passing-orange)
![RGB Passing](https://img.shields.io/badge/rgb-passing-rgb) 


*Demo Decentralized Exchange for RGB assets (RGB20)*

⚠️**We moved our development to our Core internal for Web app**

⚠️**Demo DEX for users test on the Testnet, Signet & Regtest**

⚠️**Don't use this demo software for mainnet**

⚠️**Contributions are welcome**

### About Bitswap Demo

Bitswap demo is a project designed to show users, developers and potentital investors about a DEX built on the top of Bitcoin using RGB with Client-Side-Validation.

Initially, we released the first [PoC](https://bitswap-bifi.github.io/Bitswap-PoC/), which showed how would work only swaps via BTC<>USDT

### How Works

The user can participate of Pools for earn BTC, tokens and USDT how on Uniswap

How there's no lunch free, you can lose sats in these scenarios:

- Illiquid asset

- Stop Loss

- Channel Liquidity on Lightning Network

- DLC fail

### Swaps (On-chain and Off-chain)

- Alice create invoice (RGB or LN) for Bob receive USDT

- Bob receive USDT tokens of Alice

- Alice receive BTC of Bob

- DLCs verify price settled and solve 2-of-2 pairs
  
## Development

[Run](https://github.com/BitSwap-BiFi/Bitswap-demo/blob/main/doc/development.md)

## Documentation for run DEX

 [Run](https://github.com/BitSwap-BiFi/Bitswap-demo/blob/main/doc/run.md)
 
## Documentation about DEX

[Official Documentation](https://github.com/BitSwap-BiFi/bitswap-docs)

## Ossification DEX

In this Core, we'll not integrate with Sidechains non built on RGB and altcoins beyond RGB, Bitcoin, Prime, Liquid, Bifrost, Payjoin, RGB wallets and Lightning Network.

## FAQ

Check [here](https://github.com/BitSwap-BiFi/Bitswap-FAQ/)

## BITP (Bitswap Improvement Proposal - Similar BIPs to the Bitcoin)

[Official Specs](https://github.com/BitSwap-BiFi/BITP)

## Roadmap

- [ ] Release crates
- [x] Full CLI
- [x] Cross-platform 
- [x] Full GUI by rust
- [ ] Liquid support to CLI
- [ ] Testnet4
- [x] Full GUI by rust
- [x] Testnet4
- [ ] Fix bugs
  
## Contributors

<a align="center" href="https://github.com/BitSwap-BiFi/Bitswap-demo/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=BitSwap-BiFi/Bitswap-demo" />
</a>
