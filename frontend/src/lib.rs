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

mod generated;
mod page;

use page::*;
use generated::css_classes::C;

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
    match &model.page_model {
        PageModel::Home(_) => page::home::init(orders.proxy(Msg::Home)),
        PageModel::About(_) => page::about::init(orders.proxy(Msg::About)),
        PageModel::Skill(inner_model) =>
            page::skill::init(orders.proxy(Msg::Skill), &inner_model.skill_id),
        PageModel::Employee(inner_model) =>
            page::employee::init(orders.proxy(Msg::Employee), &inner_model.employee_id),
        PageModel::NotFound(_) => page::not_found::init(orders.proxy(Msg::NotFound)),
    }
    page::partial::header::init(orders.proxy(Msg::Header));
    page::partial::footer::init(orders.proxy(Msg::Footer));
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let page_model = &mut model.page_model;
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            *model = init(url, orders);
        },
        Msg::Home(inner_msg) => {
            if let PageModel::Home(inner_model) = page_model {
                page::home::update(
                    &mut orders.proxy(Msg::Home),
                    inner_model,
                    inner_msg,
                );
            }
        },
        Msg::Skill(inner_msg) => {
            if let PageModel::Skill(inner_model) = page_model {
                page::skill::update(
                    &mut orders.proxy(Msg::Skill),
                    inner_model,
                    inner_msg,
                );
            }
        },
        Msg::Employee(inner_msg) => {
            if let PageModel::Employee(inner_model) = page_model {
                page::employee::update(
                    &mut orders.proxy(Msg::Employee),
                    inner_model,
                    inner_msg,
                );
            }
        },
        Msg::Header(inner_msg) => {
            page::partial::header::update(
                &mut orders.proxy(Msg::Header),
                &mut model.header_model,
                inner_msg,
            );
        },
        Msg::Footer(inner_msg) => {
            page::partial::footer::update(
                &mut orders.proxy(Msg::Footer),
                &mut model.footer_model,
                inner_msg,
            )
        },
        Msg::About(_) => unimplemented!(),
        Msg::NotFound(_) => unimplemented!(),
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        C![
            IF!(not(model.is_in_prerendering()) => C.fade_in),
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match &model.page_model {
            PageModel::Home(inner_model) => page::home::view(inner_model).map_msg(Msg::Home),
            PageModel::About(inner_model) => page::about::view(inner_model).map_msg(Msg::About),
            PageModel::Skill(inner_model) => page::skill::view(inner_model).map_msg(Msg::Skill),
            PageModel::Employee(inner_model) =>
                page::employee::view(inner_model).map_msg(Msg::Employee),
            PageModel::NotFound(inner_model) => page::not_found::view(inner_model).map_msg(Msg::NotFound),
        },
        page::partial::header::view(&model.header_model).map_msg(Msg::Header),
        page::partial::footer::view(&model.footer_model).map_msg(Msg::Footer),
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::start("app", init, update, view);
    log!("App started.");
}
