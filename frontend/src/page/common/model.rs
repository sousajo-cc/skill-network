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