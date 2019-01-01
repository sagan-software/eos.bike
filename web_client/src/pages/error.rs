use crate::Context;
use contract::{Slug, Url};
use eosio::*;
use eosio_rpc::Builder;
use wasm_bindgen::JsValue;
use web_sys::Element;

pub struct Page<'a> {
    msg: &'a str,
}

impl<'a> super::Page<&'a str> for Page<'a> {
    fn new(msg: &'a str) -> Self {
        Page { msg }
    }

    fn render(&self, ctx: &Context) -> Result<Element, JsValue> {
        let el = ctx.document.create_element("div")?;
        el.set_inner_html(self.msg);
        Ok(el)
    }
}
