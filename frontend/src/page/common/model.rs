use crate::page::*;

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
    pub search_query: String,
    pub matched_skills: Vec<Skill>,
}

impl Model {
    pub fn new(url: Url) -> Self {
        Model {
            base_url: url.to_base_url(),
            page: Page::new(url),
            scroll_history: ScrollHistory::new(),
            menu_visibility: Visibility::Hidden,
            in_prerendering: is_in_prerendering(),
            search_query: String::new(),
            matched_skills: Vec::new(),
        }
    }
}