use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use async_graphql::Data;
use clap::{Parser, ValueHint};
use serde::{Deserialize, Serialize};

const DEFAULT_DATADIR: &str = "~/.dlc-amm-server";
const API_DIR: &str = "api";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]

pub enum DatabaseType {
    Sqlite,
}

impl FromStr for DatabaseType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sqlite" => Ok(DatabaseType::Sqlite),
            _ => Err("Unknown db type".to_owned()),
        }
    }
}

#[derive(Parser, Clone, Eq, PartialEq, Debug)]
#[command(author, version, about)]
pub struct Opts {
    pub bitcoind_rpc_username: String,
    pub bitcoind_rpc_password: String,
    pub bitcoind_rpc_port: u16,
    pub bitcoind_rpc_host: String,
    pub bitcoind_wallet: Option<String>,
    pub oracle_host: String,
    #[clap(short, long, global = true, default_value = DEFAULT_DATADIR, env = "DLC_AMM_SERVER_DATADIR", value_hint = ValueHint::DirPath)]
    pub datadir: String,

    pub db_type: DatabaseType,
}

impl Opts {
    pub fn get_db_url(&self) -> String {
        let p = PathBuf::from(&self.datadir);
        match self.db_type {
            DatabaseType::Sqlite => Path::new("sqlite://")
                .join(p)
                .join(API_DIR)
                .into_os_string()
                .into_string()
                .unwrap(),
        }
    }
