# Bitswap Core  ⚡ 💱

[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)


*Decentralized Exchange for RGB assets (RGB20)*

⚠️**Alpha Software and WIP**

⚠️**Unstable Software**

⚠️**Isn't ready to compile via Cargo.lock for while**


**Contributions are welcome**

### How Works?

The user can participate of Pools and earn BTC, tokens and USDT like on Uniswap

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

### Features

- [Payjoin](https://payjoin.org/)
- Universal swaps, atomic swaps via on-chain and Lightning Network
- P2P Swaps without intermediary with [DLCs](https://github.com/p2pderivatives/rust-dlc)
- Taproot
- LDK and RGB Lightning Node
- [Prime](https://github.com/LNP-BP/layer1) & [Liquid](https://liquid.net/)
- [Bifrost](https://www.rgbfaq.com/glossary/bifrost)
- CLI powered by RGB CLI 
- BOLT12
- Non custodial via on-chain and Lightning Network
- Privacy
- Non KYC
- Multipeer channel
- Taproot Channels
- DLC Off chain thought Lightning swaps
- AMM like Uniswap
- LSP for RGB20 Assets
- PTLCs
- Musig2
- [Bitlight Wallet](https://bitlightlabs.com/) & [Bitmask](https://bitmask.app/)
- Contractum for complex contracts
- DCA decentralized thought USDT
- RGB Proxy
- Support for Tether and other Stablecoins 

## Roadmap

- [x] Compile to Cargo.lock (WIP)
- [x] RGB v0.11 (WIP)
- [ ] Demo DEX via CLI (Beta version - internal tests)

## Development

[Run](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/doc/development.md)

## Documentation for run DEX

 [Run](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/doc/run.md)
 
## Documentation about DEX

[Official Documentation](https://github.com/BitSwap-BiFi/bitswap-docs)

## License ⚠️

The code is licensed under either:

-  [Business Source License 1.1](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/LICENSE.md)
-  This licensed under [BSL 1.1](https://mariadb.com/bsl11/), a "source available" license which automatically turns into an open source license after 4 years, see [LICENSE](https://github.com/BitSwap-BiFi/Bitswap-core/blob/main/LICENSE.md) and [BSL FAQs](https://mariadb.com/bsl-faq-mariadb/) for details. 


## Ossification DEX

In this Core, we'll not integrate with Sidechains non built on RGB and altcoins beyond RGB, Bitcoin, Prime, Liquid, Bifrost, Payjoin, RGB wallets and Lightning Network.

## FAQ

Check [here](https://github.com/BitSwap-BiFi/Bitswap-FAQ/)


## Contributors

<a align="center" href="https://github.com/BitSwap-BiFi/Bitswap-core/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=BitSwap-BiFi/Bitswap-core" />
</a>
