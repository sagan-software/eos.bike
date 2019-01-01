#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod constants;
mod context;
mod pages;
mod route;
mod scatter;
mod utils;

use crate::context::{Context, ScatterState};
use crate::pages::Page;
use crate::route::Route;
use crate::scatter::Scatter;
use cfg_if::cfg_if;
use eosio::*;
use futures::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, Event, EventTarget, Location, Window};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    crate::utils::set_panic_hook();

    let mut ctx = Context::new()?;
    let pathname = ctx.location.pathname().expect("unable to get pathname");

    // .and_then(|scatter| {
    //     let rf = RequiredFields {
    //         accounts: Some(vec![Network {
    //             chain_id: Some(crate::CHAIN_ID),
    //             protocol: None,
    //             blockchain: Some("eos"),
    //             host: None,
    //             port: None,
    //         }]),
    //     };
    //     scatter.get_identity(rf).map(|id| (scatter, id))
    // })

    let page = match Route::from(pathname) {
        Route::Home => pages::home::Page::new(()).render(&ctx)?,
        Route::Url(slug) => pages::url::Page::new(&slug).render(&ctx)?,
        Route::UrlInfo(slug) => pages::url_info::Page::new(&slug).render(&ctx)?,
        Route::Error(msg) => pages::error::Page::new(msg).render(&ctx)?,
    };

    Scatter::use_eos();

    let body = ctx.document.body().expect("document should have a body");
    body.append_child(&page)?;

    let history = ctx.window.history()?;
    let router = crate::route::Agent {
        window: ctx.window.clone(),
        history,
    };
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

    // let fut = Scatter::connect(crate::APP_NAME)
    //     .map(move |scatter| {
    //         context.scatter = Some(scatter);
    //         JsValue::NULL
    //     })
    //     .map_err(|_| JsValue::NULL);
    // future_to_promise(fut);
    let ctx2 = ctx.clone();
    let a = Closure::wrap(Box::new(move || {
        web_sys::console::log_2(
            &JsValue::from_str("!!! is not_asked"),
            &JsValue::from_bool(ctx2.scatter.is_not_asked()),
        );
    }) as Box<dyn Fn()>);
    web_sys::window().expect("balls")
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
        a.forget();

    ctx.load_scatter();
    Ok(())
}
