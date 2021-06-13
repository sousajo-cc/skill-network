use crate::page::*;

// TODO: separate into a model for each page
pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
    // home
    pub search_query: String,
    pub matched_skills: Vec<Skill>,
    // skill page
    pub skill: Option<Skill>,
    pub matched_employees: Vec<Employee>,
    pub error: Option<String>,
    //employee page
    pub employee: Option<Employee>,
    pub employee_skills: Vec<Skill>,
    pub error_employee: Option<String>,
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
            skill: None,
            matched_employees: Vec::new(),
            error: None,
            employee: None,
            employee_skills: Vec::new(),
            error_employee: None,
        }
    }
}
