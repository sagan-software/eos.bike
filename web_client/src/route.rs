use contract::Slug;
use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::{History, Window};

pub enum Route<'a> {
    Home,
    Url(Slug),
    // UrlInfo(Slug),
    Error(&'a str),
}

impl From<&'_ str> for Route<'_> {
    fn from(pathname: &str) -> Self {
        match pathname.get(1..) {
            Some(slug) => {
                if slug == "" {
                    Route::Home
                } else {
                    match eosio::sys::string_to_name(slug) {
                        Ok(num) => Route::Url(num.into()),
                        Err(_) => Route::Error("invalid slug"),
                    }
                }
            }
            None => Route::Home,
        }
    }
}

impl From<String> for Route<'_> {
    fn from(pathname: String) -> Self {
        Self::from(pathname.as_str())
    }
}

pub struct Agent {
    pub window: Window,
    pub history: History,
    // onpopstate: &'a Function,
}

impl Agent {
    pub fn push_state(&self, title: &str, url: &str) -> Result<(), JsValue> {
        self.history
            .push_state_with_url(&JsValue::NULL, title, Some(url))
    }
}

// TODO impl Drop
