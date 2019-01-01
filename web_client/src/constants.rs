use eosio::{n, AccountName};

pub const APP_NAME: &str = "eos.bike";

pub const CHAIN_ID: &str = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f";

pub const NODE: &str = "https://127.0.0.1:8889";

pub const ACCOUNT: AccountName = AccountName(n!(urlshortener));
