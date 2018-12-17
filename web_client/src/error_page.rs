use crate::Context;
use contract::{Slug, Url};
use eosio::*;
use eosio_rpc::Builder;
use wasm_bindgen::JsValue;
use web_sys::Element;

pub fn render(state: &Context, msg: &str) -> Result<Element, JsValue> {
    let el = state.document.create_element("div")?;
    el.set_inner_html(msg);
    Ok(el)
}
