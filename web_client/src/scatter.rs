use eosio::{AccountName, Action};
use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequiredFields<'rf> {
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Network<'rf>>>,
}

impl PartialEq for RequiredFields<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.accounts == other.accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network<'n> {
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<&'n str>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<&'n str>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<&'n str>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<&'n str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

impl PartialEq for Network<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id
            && self.protocol == other.protocol
            && self.blockchain == other.blockchain
            && self.host == other.host
            && self.port == other.port
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Account {
    pub name: AccountName,
    pub authority: String,
    pub blockchain: String,
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        self.name == other.name
            && self.authority == other.authority
            && self.blockchain == other.blockchain
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub hash: String,
    pub kyc: bool,
    pub name: String,
    pub public_key: String,
    pub accounts: Vec<Account>,
}

impl Identity {
    pub fn account_name(&self) -> Option<AccountName> {
        match self.accounts.first() {
            Some(account) => Some(account.name.clone()),
            None => None,
        }
    }
}

impl PartialEq for Identity {
    fn eq(&self, other: &Identity) -> bool {
        self.hash == other.hash
            && self.kyc == other.kyc
            && self.name == other.name
            && self.public_key == other.public_key
            && self.accounts == other.accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Error {
    NotConnected,
    Locked,
    Rejected,
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub actions: Vec<serde_json::Value>,
}

impl<Data> From<Action<Data>> for Transaction
where
    Data: Serialize,
{
    fn from(action: Action<Data>) -> Transaction {
        let serialized_action = serde_json::to_value(&action).unwrap();
        Transaction {
            actions: vec![serialized_action],
        }
    }
}

impl<Data> From<Vec<Action<Data>>> for Transaction
where
    Data: Serialize,
{
    fn from(actions: Vec<Action<Data>>) -> Transaction {
        let mut serialized_actions = Vec::new();
        for action in &actions {
            serialized_actions.push(serde_json::to_value(&action).unwrap());
        }
        Transaction {
            actions: serialized_actions,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushedTransaction {
    pub transaction_id: String,
}

mod bindings {
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsStatic;

    #[wasm_bindgen(module = "scatterjs-core")]
    extern "C" {
        #[wasm_bindgen(js_name = default)]
        pub type Scatter;

        #[wasm_bindgen(js_namespace = default, js_name = plugins)]
        pub fn eos_plugin(plugin: EosPlugin);

        #[wasm_bindgen(js_namespace = default)]
        pub static scatter: Scatter;

        #[wasm_bindgen(method)]
        pub fn connect(this: &Scatter, appname: &str) -> Promise;

        #[wasm_bindgen(method, js_name = getIdentity)]
        pub fn get_identity(this: &Scatter, required_fields: JsValue) -> Promise;

        #[wasm_bindgen(method, js_name = forgetIdentity)]
        pub fn forget_identity(this: &Scatter) -> Promise;

        // #[wasm_bindgen(method, getter)]
        // pub fn identity(this: &Scatter) -> Option<JsValue>;

        #[wasm_bindgen(method)]
        pub fn suggest_network(this: &Scatter, network: JsValue) -> Promise;

        #[wasm_bindgen(method)]
        pub fn eos(this: &Scatter, network: JsValue, eos: &JsValue, config: JsValue) -> Eos;
    }

    #[wasm_bindgen(module = "scatterjs-plugin-eosjs")]
    extern "C" {
        #[wasm_bindgen(js_name = default)]
        pub type EosPlugin;

        #[wasm_bindgen(constructor, js_class = default)]
        pub fn new() -> EosPlugin;
    }

    #[wasm_bindgen(module = "eosjs")]
    extern "C" {
        #[wasm_bindgen(js_name = default)]
        pub type Eos;

        #[wasm_bindgen(js_name = default)]
        pub static eos_constructor: JsValue;

        #[wasm_bindgen(method)]
        pub fn transaction(this: &Eos, transaction: JsValue) -> Promise;
    }
}

pub struct Scatter<'s>(&'s bindings::Scatter);

impl<'s> Scatter<'s> {
    pub fn use_eos() {
        self::bindings::eos_plugin(self::bindings::EosPlugin::new());
    }

    pub fn connect(appname: &str) -> impl Future<Item = Self, Error = JsValue> {
        let promise = self::bindings::scatter.connect(appname);
        JsFuture::from(promise).and_then(|value| {
            let connected = value.as_bool().unwrap_or_else(|| false);
            if connected {
                future::ok(Scatter(&self::bindings::scatter))
            } else {
                future::err(JsValue::from_str("could not connect"))
            }
        })
    }

    pub fn get_identity<'rf>(
        &self,
        required_fields: RequiredFields<'rf>,
    ) -> impl Future<Item = Identity, Error = JsValue> {
        let required_fields_value = JsValue::from_serde(&required_fields).unwrap();
        let promise = self.0.get_identity(required_fields_value);
        JsFuture::from(promise).and_then(|value| {
            let identity = value.into_serde::<Identity>().unwrap();
            future::ok(identity)
        })
    }

    pub fn push_transaction(
        &self,
        network: &Network,
        transaction: &Transaction,
        config: &EosJsConfig,
    ) -> impl Future<Item = JsValue, Error = JsValue> {
        let network_value = JsValue::from_serde(network).unwrap();
        let config_value = JsValue::from_serde(config).unwrap();
        let eos = self
            .0
            .eos(network_value, &bindings::eos_constructor, config_value);
        let transaction_value = JsValue::from_serde(transaction).unwrap();
        let promise = eos.transaction(transaction_value);
        JsFuture::from(promise)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EosJsConfig<'a> {
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<&'a str>,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider: Option<Vec<&'a str>>,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_in_seconds: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<bool>,
}
