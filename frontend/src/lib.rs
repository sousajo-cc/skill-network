//#![deny(warnings)]
#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

extern crate console_error_panic_hook;

use seed::{*, prelude::*};

use generated::css_classes::C;

mod generated;
mod page;

use page::Model;

#[macro_use]
extern crate serde_derive;

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let orders = orders.proxy(Msg::Home);
    page::home::init(url, orders)
}

#[derive(Debug)]
pub enum Msg {
    Home(page::home::Msg),
    About(page::about::Msg),
    NotFound(page::not_found::Msg),
    Header(page::partial::header::Msg),
    Footer(page::partial::footer::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Home(inner_msg) =>
            page::home::update(&mut orders.proxy(Msg::Home), model, inner_msg),
        _ => unimplemented!()
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    use page::Page;

    div![
        C![
            IF!(not(model.in_prerendering) => C.fade_in),
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view(&model).map_msg(|msg| Msg::Home(msg)),
            Page::About => page::about::view().map_msg(|msg| Msg::About(msg)),
            Page::NotFound => page::not_found::view().map_msg(|msg| Msg::NotFound(msg)),
        },
        page::partial::header::view(model).map_msg(|msg| Msg::Header(msg)),
        page::partial::footer::view().map_msg(|msg| Msg::Footer(msg)),
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::start("app", init, update, view);
    log!("App started.");
}
