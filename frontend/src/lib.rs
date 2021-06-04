#![deny(warnings)]
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
use page::Page;

#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
pub enum Msg {
    Home(page::home::Msg),
    About(page::about::Msg),
    Skill(page::skill::Msg),
    NotFound(page::not_found::Msg),
    Header(page::partial::header::Msg),
    Footer(page::partial::footer::Msg),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let model = Model::new(url);
    match model.page {
        Page::Home => page::home::init(orders.proxy(Msg::Home)),
        Page::About => page::about::init(orders.proxy(Msg::About)),
        Page::Skill => page::skill::init(orders.proxy(Msg::Skill)),
        Page::NotFound => page::not_found::init(orders.proxy(Msg::NotFound)),
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Home(inner_msg) =>
            page::home::update(&mut orders.proxy(Msg::Home), model, inner_msg),
        Msg::Skill(inner_msg) =>
            page::skill::update(&mut orders.proxy(Msg::Skill), model, inner_msg),
        _ => unimplemented!()
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        C![
            IF!(not(model.in_prerendering) => C.fade_in),
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view(model).map_msg(Msg::Home),
            Page::About => page::about::view().map_msg(Msg::About),
            Page::Skill => page::skill::view(model).map_msg(Msg::Skill),
            Page::NotFound => page::not_found::view().map_msg(Msg::NotFound),
        },
        page::partial::header::view(model).map_msg(Msg::Header),
        page::partial::footer::view().map_msg(Msg::Footer),
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::start("app", init, update, view);
    log!("App started.");
}
