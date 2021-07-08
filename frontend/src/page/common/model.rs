use crate::page::*;

pub struct Model {
    pub header_model: partial::header::Model,
    pub footer_model: partial::footer::Model,
    pub page_model: PageModel,
}

impl Model {
    pub fn new(url: Url) -> Self {
        Self {
            header_model: partial::header::Model::new(&url),
            footer_model: partial::footer::Model,
            page_model: PageModel::new(url),
        }
    }

    pub fn is_in_prerendering(&self) -> bool {
        self.header_model.in_prerendering
    }
}

pub enum PageModel {
    Home(home::Model),
    Skill(InnerModel),
    Employee(InnerModel),
    About(about::Model),
    NotFound(not_found::Model),
}

impl PageModel {
    pub fn new(url: Url) -> Self {
        let mut inner_model = InnerModel::new(&url);
        let page = Page::new(url.clone());
        match page {
            Page::Home => Self::Home(home::Model::new(&url)),
            Page::Skill(id) => {
                inner_model.skill_id = id;
                Self::Skill(inner_model)
            },
            Page::Employee(id) => {
                inner_model.employee_id = id;
                Self::Employee(inner_model)
            },
            Page::About => Self::About(about::Model),
            Page::NotFound => Self::NotFound(not_found::Model),
        }
    }
}

pub struct InnerModel {
    pub base_url: Url,
    // skill page
    pub skill_id: String,
    pub skill: Option<Skill>,
    pub matched_employees: Vec<Employee>,
    pub error: Option<String>,
    // employee page
    pub employee_id: String,
    pub employee: Option<Employee>,
    pub employee_skills: Vec<Skill>,
    pub error_employee: Option<String>,
}

impl InnerModel {
    pub fn new(url: &Url) -> Self {
        Self {
            base_url: url.to_base_url(),
            skill_id: String::new(),
            skill: None,
            matched_employees: Vec::new(),
            error: None,
            employee_id: String::new(),
            employee: None,
            employee_skills: Vec::new(),
            error_employee: None,
        }
    }
}
