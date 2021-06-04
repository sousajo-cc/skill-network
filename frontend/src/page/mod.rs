pub mod common;

pub mod home;
pub mod about;
pub mod skill;
pub mod not_found;
pub mod partial;

use seed::{prelude::*, *};
use crate::generated::css_classes::C;
pub use common::*;
use fixed_vec_deque::FixedVecDeque;

const TITLE_SUFFIX: &str = "Company";
const ABOUT: &str = "about";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const _STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";
const MAIL_TO_US: &str = "mailto:company@company.com";

#[derive(Clone, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    Skill(String),
    NotFound,
}

impl Page {
    pub fn new(mut url: Url) -> Self {
        let page = match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [ABOUT] => Self::About,
            ["skill", id] => Self::Skill(id.to_string()),
            _ => Self::NotFound,
        };
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

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");
    user_agent == USER_AGENT_FOR_PRERENDERING
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn _asset_path(asset: &str) -> String {
    format!("{}/{}", _STATIC_PATH, asset)
}

