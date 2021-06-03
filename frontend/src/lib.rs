//#![deny(warnings)]
#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

extern crate console_error_panic_hook;

use fixed_vec_deque::FixedVecDeque;
use seed::{*, prelude::*};

use generated::css_classes::C;

mod generated;
mod page;

#[macro_use]
extern crate serde_derive;

// Page title
const TITLE_SUFFIX: &str = "Company";
const MAIL_TO_US: &str = "mailto:company@company.com";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub enum Model {
    Home(page::home::Model),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let orders = orders.proxy(Msg::Home);
    let page = page::home::init(url, orders);
    Model::Home(page)
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {}", TITLE_SUFFIX)),
            _ => (Self::NotFound, format!("404 - {}", TITLE_SUFFIX)),
        };
        document().set_title(&title);
        page
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
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
    match (model, msg) {
        (Model::Home(inner_model), Msg::Home(inner_msg)) =>
            page::home::update(&mut orders.proxy(Msg::Home), inner_model, inner_msg),
        _ => unimplemented!()
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    match model {
        Model::Home(inner_model) =>
            div![
            C![
                IF!(not(inner_model.in_prerendering) => C.fade_in),
                C.min_h_screen,
                C.flex,
                C.flex_col,
            ],
            match inner_model.page {
                Page::Home => page::home::view(&inner_model).map_msg(|msg| Msg::Home(msg)),
                Page::About => page::about::view().map_msg(|msg| Msg::About(msg)),
                Page::NotFound => page::not_found::view().map_msg(|msg| Msg::NotFound(msg)),
            },
            page::partial::header::view(inner_model).map_msg(|msg| Msg::Header(msg)),
            page::partial::footer::view().map_msg(|msg| Msg::Footer(msg)),
        ]
    }
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::start("app", init, update, view);
    log!("App started.");
}
