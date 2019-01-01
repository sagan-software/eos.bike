use crate::constants::{ACCOUNT, NODE};
use crate::Context;
use contract::{Slug, Url};
use eosio::*;
use eosio_rpc::Builder;
use futures::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::Element;

fn get_url(slug: &Slug) -> impl Future<Item = Option<Url>, Error = ::eosio_rpc::Error> {
    let client = ::eosio_rpc::WebSysClient::new(NODE).unwrap();
    eosio_rpc::chain::get_table_rows(ACCOUNT.into(), ACCOUNT.into(), Url::TABLE_NAME.into())
        .lower_bound(slug.as_u64())
        .upper_bound(slug.as_u64() + 1)
        .limit(1)
        .fetch(&client)
        .map(|mut response| response.rows.pop())
}

pub struct Page<'a> {
    pub slug: &'a Slug,
    pub promise: Option<Promise>,
}

impl<'a> super::Page<&'a Slug> for Page<'a> {
    fn new(slug: &'a Slug) -> Self {
        // let location = context.location.clone();
        // let cb = Closure::wrap(Box::new(move |value: JsValue| {
        //     web_sys::console::log_2(&JsValue::from_str("RESPONSE"), &value);
        //     match value.into_serde::<Option<Url>>() {
        //         Ok(response) => match response {
        //             Some(url) => {
        //                 web_sys::console::log_1(&JsValue::from_str("SUCCESS"));
        //                 location.assign(&url.url);
        //             }
        //             None => {
        //                 web_sys::console::log_1(&JsValue::from_str("NOT FOUND"));
        //             }
        //         },
        //         Err(err) => {
        //             web_sys::console::log_1(&JsValue::from_str("ERROR"));
        //         }
        //     }
        // }) as Box<dyn FnMut(JsValue)>);
        // let promise = future_to_promise(
        //     get_url(slug)
        //         .map(|info| JsValue::from_serde(&info).unwrap())
        //         .map_err(|_| JsValue::from_str("error")),
        // )
        // .then(&cb);
        // cb.forget();
        Self {
            slug,
            promise: None,
        }
    }
    fn render(&self, ctx: &Context) -> Result<Element, JsValue> {
        render_loading(ctx)
    }
}

fn render_loading(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.document.create_element("div")?;
    el.set_inner_html("Loading...");
    Ok(el)
}

fn render_error(ctx: &Context) -> Result<Element, JsValue> {
    let el = ctx.document.create_element("div")?;
    el.set_inner_html("Error!");
    Ok(el)
}
