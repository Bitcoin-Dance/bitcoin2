use lazy_static::lazy_static;
use sqlx::{migrate::Migrator, types::time::PrimitiveDateTime};

use std::env;
use time::macros::datetime;

macro_rules! account_id {
    ($last_byte:expr) => {{
        let mut array = [0u8; 20];
        array[19] = $last_byte;
        array
    }};
}
pub const LEGACY_ACCOUNT: [u8; 20] = account_id!(0);
pub const DEFAULT_GAS_LIMIT: i64 = 21000;
pub const LAST_LEGACY_BLOCK_TIMESTAMP: PrimitiveDateTime = datetime!(2024-04-19 08:05:33);
pub const LAST_LEGACY_BLOCK_NUMBER: i64 = 839999;
pub const CHAIN_ID: i64 = 178;
pub const SYSTEM_ADDRESS: [u8; 20] = [0; 20];

// ethers.FunctionFragment.getSelector('upgradeByMessage', ['string', 'bytes'])
pub const UPGRADE_BY_MESSAGE: [u8; 4] = *b"\xe6\x0b\x06\x0d";
pub static MIGRATOR: Migrator = sqlx::migrate!();

pub enum Env {
    Production,
    Development,
}

lazy_static! {
    pub static ref ENV: Env = if env::var("ENV").unwrap_or("".to_string()) == "production" {
        Env::Production
    } else {
        Env::Development
    };
    pub static ref PORT: u16 = env::var("PORT")
        .and_then(|port| Ok(port.parse().unwrap_or(3000)))
        .unwrap();
    pub static ref LETS_ENCRYPT_EMAILS: Vec<String> = env::var("LETS_ENCRYPT_EMAILS")
        .and_then(|emails| Ok(emails
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .clone()))
        .unwrap_or(vec![]);
    pub static ref LETS_ENCRYPT_DOMAINS: Vec<String> = env::var("LETS_ENCRYPT_DOMAINS")
        .and_then(|emails| Ok(emails
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .clone()))
        .unwrap_or(vec![]);
}

const _LEGACY_ACCOUNT: [u8; 20] = account_id!(0x00);
