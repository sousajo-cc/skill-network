#![deny(warnings)]
#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

extern crate console_error_panic_hook;

use seed::{prelude::*, *};

use generated::css_classes::C;

mod generated;
mod page;

use page::{Model, Page};

#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Home(page::home::Msg),
    About(page::about::Msg),
    Skill(page::skill::Msg),
    Employee(page::employee::Msg),
    NotFound(page::not_found::Msg),
    Header(page::partial::header::Msg),
    Footer(page::partial::footer::Msg),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    let model = Model::new(url);
    let inner_model = match &model {
        Model::Home(m) => m,
        Model::Skill(m) => m,
        Model::Employee(m) => m,
    };
    match &inner_model.page {
        Page::Home => page::home::init(orders.proxy(Msg::Home)),
        Page::About => page::about::init(orders.proxy(Msg::About)),
        Page::Skill(id) => page::skill::init(orders.proxy(Msg::Skill), id),
        Page::Employee(id) => {
            page::employee::init(orders.proxy(Msg::Employee), id)
        },
        Page::NotFound => page::not_found::init(orders.proxy(Msg::NotFound)),
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            *model = init(url, orders);
        },
        Msg::Home(inner_msg) => {
            if let Model::Home(inner_model) = model {
                page::home::update(
                    &mut orders.proxy(Msg::Home),
                    inner_model,
                    inner_msg,
                );
            }
        },
        Msg::Skill(inner_msg) => {
            if let Model::Skill(inner_model) = model {
                page::skill::update(
                    &mut orders.proxy(Msg::Skill),
                    inner_model,
                    inner_msg,
                );
            }
        },
        Msg::Employee(inner_msg) => {
            if let Model::Employee(inner_model) = model {
                page::employee::update(
                    &mut orders.proxy(Msg::Employee),
                    inner_model,
                    inner_msg,
                );
            }
        },
        _ => unimplemented!(),
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    let model = match model {
        Model::Home(m) => m,
        Model::Skill(m) => m,
        Model::Employee(m) => m,
    };
    div![
        C![
            IF!(not(model.in_prerendering) => C.fade_in),
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match &model.page {
            Page::Home => page::home::view(model).map_msg(Msg::Home),
            Page::About => page::about::view().map_msg(Msg::About),
            Page::Skill(_) => page::skill::view(model).map_msg(Msg::Skill),
            Page::Employee(_) =>
                page::employee::view(model).map_msg(Msg::Employee),
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
