use crate::constants::{ACCOUNT, APP_NAME, CHAIN_ID};
use crate::scatter::{Network, RequiredFields, Scatter, Transaction};
use crate::Context;
use contract::Slug;
use eosio::*;
use futures::future::{self, Future};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Document, Element, Event, EventTarget, Location, Window};

pub struct Page {}

impl super::Page<()> for Page {
    fn new(_params: ()) -> Self {
        Self {}
    }

    fn render(&self, ctx: &Context) -> Result<Element, JsValue> {
        let el = ctx.document.create_element("div")?;
        el.set_class_name("home");

        let logo = render_logo(ctx)?;
        el.append_child(logo.as_ref())?;

        let form = render_form(ctx)?;
        el.append_child(form.as_ref())?;

        Ok(el)
    }
}

fn render_logo(state: &Context) -> Result<Element, JsValue> {
    let el = state.document.create_element("a")?;
    el.set_class_name("logo");
    el.set_inner_html("eos.bike");
    Ok(el)
}

fn random_slug() -> Slug {
    let chars = b"abcdefghijklmnopqrstuvwxyz12345";
    let chars_len = chars.len() as f64;
    let slug_len = 5_usize;
    let mut slug_string = String::with_capacity(slug_len);
    for i in 0..5 {
        let c_pos = (js_sys::Math::random() * chars_len) as usize;
        let c = chars[c_pos];
        slug_string.push(char::from(c));
    }
    Slug::from_string(slug_string).unwrap()
}

fn render_form(state: &Context) -> Result<Element, JsValue> {
    let el = state.document.create_element("form")?;
    el.set_class_name("url-form");

    let input = render_input(state)?;
    el.append_child(input.as_ref())?;
    let button = render_button(state)?;
    el.append_child(button.as_ref())?;

    let cb = Closure::wrap(Box::new(move |e: Event| {
        e.prevent_default();
        let i = JsCast::dyn_ref::<web_sys::HtmlInputElement>(&input).unwrap();
        let value = i.value();
        let f = Scatter::connect(APP_NAME)
            .and_then(|scatter| {
                let rf = RequiredFields {
                    accounts: Some(vec![Network {
                        chain_id: Some(CHAIN_ID),
                        protocol: None,
                        blockchain: Some("eos"),
                        host: None,
                        port: None,
                    }]),
                };
                scatter.get_identity(rf).map(|id| (scatter, id))
            })
            .and_then(|(scatter, id)| {
                let shorten = ::contract::ShortenAction {
                    id: random_slug(),
                    url: value,
                    account: n!(alice).into(),
                };
                let action = shorten.to_action(
                    ACCOUNT.into(),
                    vec![Authorization::active(n!(alice).into())],
                );

                scatter.push_transaction(
                    &Network {
                        chain_id: Some(CHAIN_ID),
                        protocol: Some("https"),
                        blockchain: Some("eos"),
                        host: Some("127.0.0.1"),
                        port: Some(8889),
                    },
                    &Transaction {
                        actions: vec![serde_json::to_value(&action).unwrap()],
                    },
                )
            })
            .map(|t| {
                web_sys::console::log_1(&JsValue::from_str("!!!"));
                t
            })
            .map_err(|_| JsValue::from_str("error"));
        // let b = Closure::wrap(Box::new(move |connected| {
        // }) as Box<dyn FnMut(JsValue)>);
        future_to_promise(f);
        // b.forget();
    }) as Box<dyn FnMut(_)>);
    let et: &EventTarget = el.as_ref();
    et.add_event_listener_with_callback("submit", cb.as_ref().unchecked_ref())?;
    cb.forget();

    Ok(el)
}

fn render_input(state: &Context) -> Result<Element, JsValue> {
    let el = state.document.create_element("input")?;
    if let Some(input) = JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el) {
        input.set_placeholder("Paste a link to shorten it");
        input.set_required(true);
        input.set_autofocus(true);
        input.set_type("url");
    }
    Ok(el)
}

fn render_button(state: &Context) -> Result<Element, JsValue> {
    let el = state.document.create_element("button")?;
    el.set_attribute("type", "submit")?;
    el.set_inner_html("Shorten");
    Ok(el)
}
