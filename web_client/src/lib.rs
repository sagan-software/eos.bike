#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod error_page;
mod home_page;
mod route;
mod scatter;
mod url_page;
mod utils;
mod wallet;

use crate::route::Route;
use cfg_if::cfg_if;
use eosio::*;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Event, EventTarget, Location, Window};

pub const CHAIN_ID: &str = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f";
pub const NODE: &str = "https://127.0.0.1:8889";
pub const ACCOUNT: AccountName = AccountName(n!(urlshortener));

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub struct Context<'a> {
    pub window: &'a Window,
    pub document: &'a Document,
    pub location: &'a Location,
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    crate::utils::set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let location = window.location();
    let pathname = location.pathname().expect("unable to get pathname");
    // web_sys::console::log_1(&pathname.into());

    let context = Context {
        window: &window,
        document: &document,
        location: &location,
    };

    let page = match Route::from(pathname) {
        Route::Home => crate::home_page::HomePage::render(&context)?,
        Route::Url(slug) => crate::url_page::UrlPage::new(&context, &slug).render()?,
        Route::Error(msg) => crate::error_page::render(&context, msg)?,
    };

    body.append_child(&page)?;

    let history = window.history()?;
    let router = crate::route::Agent { window, history };
    // router.push_state("HI", "/balls");

    // let el1 = document.create_element("a")?;
    // el1.set_attribute("href", "/test1")?;
    // el1.set_inner_html("Page 1 link");
    // let a = Closure::wrap(Box::new(move |e: Event| {
    //     e.prevent_default();
    //     router.push_state("HI", "/test1");
    // }) as Box<dyn FnMut(_)>);
    // let et1: &EventTarget = el1.as_ref();
    // et1.add_event_listener_with_callback("click", a.as_ref().unchecked_ref())?;
    // a.forget();
    // body.append_child(&el1);

    // let el2 = document.create_element("a")?;
    // el2.set_attribute("href", "/test2")?;
    // el2.set_inner_html("Page 2 link");
    // body.append_child(&el2);

    Ok(())
}
