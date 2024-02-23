# Bitswap Documentation

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
    1. [Prerequisites](#prerequisites)
    2. [Installation](#installation)
3. [Usage](#usage)
    1. [Command Line Interface (CLI)](#cli)
    2. [RGB Lightning Node](#rgb-node)
4. [Testing](#testing)
5. [Contributing](#contributing)
6. [License](#license)

## 1. Introduction <a name="introduction"></a>

Provide an overview of Bitswap, explaining its purpose, benefits, and any relevant context. Briefly mention the components, including the CLI and the RGB Lightning Node.

## 2. Getting Started <a name="getting-started"></a>

Explain how to set up and get your project running, covering both the CLI and the RGB Lightning Node.

### 2.1 Prerequisites <a name="prerequisites"></a>

List the prerequisites for both the CLI and the RGB Lightning Node, including software, tools, and dependencies.

- RGB CLI
- RGB Lightning Node
- [Rust](https://www.rust-lang.org/)
- [Cargo](https://crates.io/)

### 2.2 Installation <a name="installation"></a>

Provide detailed installation instructions for both the CLI and the RGB Lightning Node. Include Git repository cloning and dependency installation.

#### CLI Installation

```bash
# Clone the repository
git clone https://github.com/BitSwap-BiFi/Bitswap-core.git

# Change to the CLI project directory
cd bitswapcore

# Install RGB, Bitswap dependencies
cargo.toml
```

#### RGB Lightning Node Installation

```bash
# Clone the RGB Lightning Node repository
git clone https://github.com/RGB-Tools/rgb-lightning-node.git

# Change to the RGB Lightning Node project directory
cd rgb-lightning-node

# Build the node
cargo build
```

## 3. Usage <a name="usage"></a>

Explain how to use both the CLI and the RGB Lightning Node.

### 3.1 Command Line Interface (CLI) <a name="cli"></a>

Provide examples and explanations of how to use the CLI. Include usage scenarios, commands, and options.

```bash
# Example CLI command
bitswap --option value
```

### 3.2 RGB Lightning Node <a name="rgb-node"></a>

Explain how to run and configure the RGB Lightning Node, including setting up network connections and managing RGB assets.

```bash
# Example RGB Lightning Node usage
./rgb-node --config config.json
```

## 4. Testing <a name="testing"></a>

Detail how to run tests for both the CLI and the RGB Lightning Node.

### 4.1 Running Tests for CLI

Provide instructions for running tests for the CLI component.

```bash
# Run CLI tests
npm test
```

### 4.2 Running Tests for RGB Lightning Node

Explain how to run tests for the RGB Lightning Node in Rust, including any test frameworks used.

```bash
# Run RGB Lightning Node tests
cargo test
```

## 6. License <a name="license"></a>

License text copyright © 2023 Bitswap, All Rights Reserved.

Terms

The Licensor hereby grants you the right to copy, modify, create derivative works, redistribute, and make non-production use of the Licensed Work. The Licensor may make an Additional Use Grant, above, permitting limited production use.

Effective on the Change Date, or the fourth anniversary of the first publicly available distribution of a specific version of the Licensed Work under this License, whichever comes first, the Licensor hereby grants you rights under the terms of the Change License, and the rights granted in the paragraph above terminate.

If your use of the Licensed Work does not comply with the requirements currently in effect as described in this License, you must purchase a commercial license from the Licensor, its affiliated entities, or authorized resellers, or you must refrain from using the Licensed Work.

All copies of the original and modified Licensed Work, and derivative works of the Licensed Work, are subject to this License. This License applies separately for each version of the Licensed Work and the Change Date may vary for each version of the Licensed Work released by Licensor.

You must conspicuously display this License on each original or modified copy of the Licensed Work. If you receive the Licensed Work in original or modified form from a third party, the terms and conditions set forth in this License apply to your use of that work.

Any use of the Licensed Work in violation of this License will automatically terminate your rights under this License for the current and all other versions of the Licensed Work.

This License does not grant you any right in any trademark or logo of Licensor or its affiliates (provided that you may use a trademark or logo of Licensor as expressly required by this License).TO THE EXTENT PERMITTED BY APPLICABLE LAW, THE LICENSED WORK IS PROVIDED ON AN “AS IS” BASIS. LICENSOR HEREBY DISCLAIMS ALL WARRANTIES AND CONDITIONS, EXPRESS OR IMPLIED, INCLUDING (WITHOUT LIMITATION) WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, AND TITLE. Bitswap hereby grants you permission to use this License’s text to license your works, and to refer to it using the trademark “Business Source License”, as long as you comply with the Covenants of Licensor below. Covenants of Licensor
