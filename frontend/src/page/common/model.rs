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
    Skill(skill::Model),
    Employee(employee::Model),
    About(about::Model),
    NotFound(not_found::Model),
}

impl PageModel {
    pub fn new(url: Url) -> Self {
        let base_url = url.to_base_url();
        let page = Page::new(url);
        match page {
            Page::Home => Self::Home(home::Model::new(base_url)),
            Page::Skill(id) => Self::Skill(skill::Model::new(base_url, id)),
            Page::Employee(id) => Self::Employee(employee::Model::new(base_url, id)),
            Page::About => Self::About(about::Model),
            Page::NotFound => Self::NotFound(not_found::Model),
        }
    }
}
