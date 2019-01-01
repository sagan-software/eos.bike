pub mod error;
pub mod home;
pub mod url;
pub mod url_info;

use crate::context::Context;
use wasm_bindgen::prelude::*;
use web_sys::Element;

pub trait Page<Params> {
    fn new(params: Params) -> Self;
    fn render(&self, ctx: &Context) -> Result<Element, JsValue>;
}
