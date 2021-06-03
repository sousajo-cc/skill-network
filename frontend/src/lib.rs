#![deny(warnings)]
#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

extern crate console_error_panic_hook;

mod generated;
mod page;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, *};
use Visibility::*;
use crate::Model::Home;

#[macro_use]
extern crate serde_derive;

// Page title
const TITLE_SUFFIX: &str = "Company";
const MAIL_TO_US: &str = "mailto:company@company.com";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

#[derive(Clone, Debug, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub skill: String,
}



fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

//enum

pub enum Model {
    Home(page::home::Model),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Home(page::home::init(url, &mut orders.proxy(Msg::HomeMsg)))
}


// ------ Page ------

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

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg {
    HomeMsg(page::home::Msg),
    AboutMsg(page::about::Msg),
    NotFound(page::not_found::Msg),
    Header(page::partial::header::Msg),
    Footer(page::partial::footer::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match (model, msg) {
        (Model::Home(inner_model), Msg::HomeMsg(inner_msg)) =>
            page::home::update(&mut orders.proxy(Msg::HomeMsg), inner_model, inner_msg),
        _ => unimplemented!()
    }
}


// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

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
                Page::Home => page::home::view(&inner_model).map_msg(|msg| Msg::HomeMsg(msg)),
                Page::About => page::about::view().map_msg(|msg| Msg::AboutMsg(msg)),
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

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");
    App::start("app", init, update, view);
    log!("App started.");
}
