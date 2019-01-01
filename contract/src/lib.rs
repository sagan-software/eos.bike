use eosio::*;

eosio_name!(Slug);

#[eosio_table(urls)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Url {
    #[primary]
    pub id: Slug,
    pub url: String,
    #[secondary]
    pub account: AccountName,
}

const SUPPORTED_PROTOCOLS: [&str; 2] = ["http://", "https://"];

pub fn is_valid_url(url: &str) -> bool {
    // Must include a TLD
    if !url.contains('.') {
        return false;
    }

    for protocol in &SUPPORTED_PROTOCOLS {
        if url.starts_with(protocol) {
            // URL must be longer than the protocol + 1 period + 2 chars
            return url.len() >= protocol.len() + 3;
        }
    }

    false
}

#[eosio_action]
pub fn shorten(id: Slug, url: String, account: AccountName) {
    require_auth(account);

    check(is_valid_url(&url), "Invalid URL supplied");

    let _self = AccountName::receiver();
    let table = Url::table(_self, _self);

    match table.find(id) {
        Some(cursor) => {
            let mut row = cursor.get().check("read");
            check(
                account == row.account,
                "ID already exists but account is not the owner",
            );
            check(url != row.url, "ID already exists but URL hasn't changed");
            row.url = url;
            cursor.modify(None, &row).check("write");
        }
        None => {
            let row = Url { id, url, account };
            table.emplace(_self, &row).check("write");
        }
    }
}

#[eosio_action]
pub fn unshorten(id: Slug, account: AccountName) {
    require_auth(account);

    let _self = AccountName::receiver();
    let table = Url::table(_self, _self);
    let cursor = table.find(id).check("no URL found with that ID");
    let row = cursor.get().check("read");
    check(
        account == row.account,
        "ID already exists but account is not the owner",
    );

    cursor.erase().check("read");
}

eosio_abi!(shorten, unshorten);
