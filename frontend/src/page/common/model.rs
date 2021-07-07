use crate::page::*;

pub enum Model {
    Home(InnerModel),
    Skill(InnerModel),
    Employee(InnerModel),
    About(InnerModel),
    NotFound(InnerModel),
}

impl Model {
    pub fn new(url: Url) -> Model {
        let inner_model = InnerModel::new(url);
        match &inner_model.page {
            Page::Home => Model::Home(inner_model),
            Page::Skill(_) => Model::Skill(inner_model),
            Page::Employee(_) => Model::Employee(inner_model),
            Page::About => Model::About(inner_model),
            Page::NotFound => Model::NotFound(inner_model),
        }
    }
}

pub struct InnerModel {
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
    // employee page
    pub employee: Option<Employee>,
    pub employee_skills: Vec<Skill>,
    pub error_employee: Option<String>,
}

impl InnerModel {
    pub fn new(url: Url) -> Self {
        InnerModel {
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
