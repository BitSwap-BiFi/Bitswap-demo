// Bitswap Core
//
// SPDX-License-Identifier: Apache-2.0, MIT LICENSE, GNU General Public License version 3
//
// Written in 2023 by 22388O
//
// Copyright (C) 2023 Bitswap. All rights reserved.
// Copyright (C) 2023 22388O. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rgb_core::{Contract, ExecutionResult};
use aluvm::{Executor, Value};
use strict_type::{Map};
use dlc::{Message, OracleInfo, ContractInfo};
use lightning::{Message, HashPayment, Invoice};

struct AmmContract {
    rgb_asset_balance: u64,
    rgb_asset_balance: u64,
    dlc_contract_balance: u64,
}

impl AmmContract {
    fn new() -> Self {
        AmmContract {
           rgb_asset_balance: 0,
            rgb_asset_balance: 0,
        }
    }

    fn add_liquidity(&mut self, rgb_asset_amount: u64, rgb_asset_amount: u64) -> ExecutionResult {
        self.rgb_asset_balance += rgb_asset_amount;
        self.rgb_asset_balance += rgb_asset_amount;
        ExecutionResult::None
    }

    fn swap(&mut self, rgb_assetamount: u64, slippage: f64) -> ExecutionResult {
        let usdt_amount = self.calculate_swap(rgb_asset_amount);

        let max_slippage = (rgb_asset_amount as f64) * slippage;
        let actual_slippage = (rgb_asset_amount as f64) - ((rgb_asset_amount as f64) * (self.rgb_asset_balance as f64) / (self.rgb_asset_balance as f64));

        if actual_slippage > max_slippage {
            // Revert the swap due to slippage exceeding the specified percentage
            ExecutionResult::None
        } else {
            self.rgb_asset_balance += rgb_asset_amount;
            self.rgb_asset_balance += rgb_asset_amount;
            ExecutionResult::Value(Value::U64(rgb_asset_amount))
        }
    }

    fn calculate_swap(&self, btc_amount: u64) -> u64 {
        //Logic AMM
       struct AMM{
    k: u64,
    x: u64,
    y: u64,
}

impl AMM {
    fn new(k: u64, x: u64, y: u64) -> Self {
        ConstantAmm { k, x, y }
    }

    fn set_x(&mut self, x: u64) {
        self.x = x;
        self.y = self.k / self.x;
    }

    fn set_y(&mut self, y: u64) {
        self.y = y;
        self.x = self.k / self.y;
    }

    fn get_x(&self) -> u64 {
        self.x
    }

    fn get_y(&self) -> u64 {
        self.y
    }
}

fn logic() {
    let mut amm = ConstantAmm::new(1000000, 100, 0);

    println!("Initial x: {}, y: {}", amm.get_x(), amm.get_y());

    amm.set_x(200);
    println!("Given x, calculated y: x: {}, y: {}", amm.get_x(), amm.get_y());

    amm.set_y(300);
    println!("Given y, calculated x: x: {}, y: {}", amm.get_x(), amm.get_y());
}

        // This example uses a simple constant ratio
        if self.rgb_asset_balance == 0 || self.rgb_asset_balance == 0 {
            0
        } else {
            (rgb_asset_amount * self.rgb_asset_balance) / self.rgb_asset_balance
        }
    }
}

fn main() {
    let mut amm_contract = AmmContract::new();

    // l iquidit provision
    let rgb_asset_liquidity = 10;
    let rgb_asset_liquidity = 100;
    let fee_liquidity= 0.3;
    amm_contract.add_liquidity(rgb_asset_liquidity, rgb_asset_liquidity, fee_liquidity);
   // token swap
    let rgb_asset_to_swap = 1
    let slippage = 0.02; // 2% maximum allowable slippage

    let result = amm_contract.swap(btc_to_swap, slippage);

    match result {
        ExecutionResult::Value(value) => {
            // User received RGB in exchange for other token
            println!("Received RGB token: {}", value);
        }
        ExecutionResult::None => {
            // Swap reverted due to slippage exceeding the specified percentage
            println!("Swap reverted due to slippage exceeding the specified percentage");
        }
    }
}


