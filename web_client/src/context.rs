use crate::constants::APP_NAME;
use crate::scatter::Scatter;
use futures::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, Event, EventTarget, Location, Window};

#[derive(Clone)]
pub struct Context {
    pub window: Window,
    pub document: Document,
    pub location: Location,
    pub scatter: ScatterState,
}

#[derive(Clone)]
pub enum ScatterState {
    NotAsked,
    Loading(Promise),
    Loaded(Scatter<'static>),
    Error(JsValue),
}

impl ScatterState {
    pub fn is_not_asked(&self) -> bool {
        match *self {
            ScatterState::NotAsked => true,
            _ => false
        }
    }
}

impl Context {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let location = window.location();

        // let history = window.history()?;
        // let router = crate::route::Agent {
        //     window: window.clone(),
        //     history,
        // };
        Ok(Self {
            window,
            document,
            location,
            scatter: ScatterState::NotAsked,
        })
    }

    pub fn load_scatter(&mut self) {
        let s = std::cell::RefCell::new(self.clone());
        let f = move |scatter| {
                // boxed.scatter = ScatterState::Loaded(scatter);
                (*(s.borrow_mut())).scatter = ScatterState::Loaded(scatter);
                JsValue::NULL
            };
        let future = Scatter::connect(APP_NAME)
            .map(f)
            .map_err(|_| JsValue::NULL);
        let promise = future_to_promise(future);
        self.scatter = ScatterState::Loading(promise);
    }
}
