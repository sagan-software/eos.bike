use contract::Slug;
use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::{History, Window};

pub enum Route<'a> {
    Home,
    Url(Slug),
    UrlInfo(Slug),
    Error(&'a str),
}

impl From<&'_ str> for Route<'_> {
    fn from(pathname: &str) -> Self {
        match pathname.get(1..) {
            Some(tail) => {
                if tail == "" {
                    return Route::Home;
                }

                let is_info = tail.ends_with("/info");
                let slug = if is_info {
                    tail.trim_end_matches("/info")
                } else {
                    tail
                };

                match eosio::sys::string_to_name(slug) {
                    Ok(num) => {
                        if is_info {
                            Route::UrlInfo(num.into())
                        } else {
                            Route::Url(num.into())
                        }
                    }
                    Err(_) => Route::Error("invalid slug"),
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

impl ToString for Route<'_> {
    fn to_string(&self) -> String {
        match *self {
            Route::Home => "/".to_string(),
            Route::Url(slug) => {
                let mut s = "/".to_string();
                s.push_str(slug.to_string().as_str());
                s
            }
            Route::UrlInfo(slug) => {
                let mut s = "/".to_string();
                s.push_str(slug.to_string().as_str());
                s.push_str("/info");
                s
            }
            Route::Error(_) => "/?error".to_string(),
        }
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
