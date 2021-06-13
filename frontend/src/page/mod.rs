pub mod common;

pub mod about;
pub mod employee;
pub mod home;
pub mod not_found;
pub mod partial;
pub mod skill;

use crate::generated::css_classes::C;
pub use common::*;
use fixed_vec_deque::FixedVecDeque;
use seed::{prelude::*, *};

const TITLE_SUFFIX: &str = "Company";
const ABOUT: &str = "about";
const SKILL: &str = "skill";
const EMPLOYEE: &str = "employee";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const _STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";
const MAIL_TO_US: &str = "mailto:company@company.com";
const BACKEND_ADDRESS: &str = "http://localhost:8000";

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Page {
    Home,
    About,
    Skill(String),
    Employee(String),
    NotFound,
}

impl Page {
    pub fn new(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [ABOUT] => Self::About,
            [SKILL, id] => Self::Skill(id.to_string()),
            [EMPLOYEE, id] => Self::Employee(id.to_string()),
            _ => Self::NotFound,
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn _about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }

    pub fn skill(self, id: &str) -> Url {
        self.base_url().add_path_part(SKILL).add_path_part(id)
    }

    pub fn employee(self, id: &str) -> Url {
        self.base_url().add_path_part(EMPLOYEE).add_path_part(id)
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

fn scroll_to_top() {
    window().scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new().top(0.),
    );
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn _asset_path(asset: &str) -> String {
    format!("{}/{}", _STATIC_PATH, asset)
}
