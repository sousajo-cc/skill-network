use crate::page::*;

pub struct Model {
    pub base_url: Url,
    pub employee_id: String,
    pub employee: Option<Employee>,
    pub employee_skills: Vec<Skill>,
    pub error: Option<String>,
    //TODO: these two below are duplicated (see home::Model)
    pub search_query: String,
    pub matched_skills: Vec<Skill>,
}

impl Model {
    pub fn new(base_url: Url, employee_id: String) -> Self {
        Self {
            base_url,
            employee_id,
            employee: None,
            employee_skills: Vec::new(),
            error: None,
            search_query: String::new(),
            matched_skills: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    EmployeeLoaded(Employee),
    SkillListLoaded(Vec<Skill>),
    RequestNOK(String),
    SearchQueryChanged(String),
    Received(Vec<Skill>),
    AddSkill(Skill),
}

pub fn init(mut orders: impl Orders<Msg>, employee_id: &str) {
    document().set_title("Employee Details");
    scroll_to_top();
    request_employee(&mut orders, employee_id);
    request_skills(&mut orders, employee_id);
}

fn request_employee(orders: &mut impl Orders<Msg>, id: &str) {
    let url = format!("{}/employee/{}", BACKEND_ADDRESS, id);
    let request = Request::new(url)
        .method(Method::Get)
        .header(Header::custom("Accept-Language", "en"));

    orders.perform_cmd(async {
        let response = fetch(request).await.expect("HTTP request failed");
        if response.status().is_ok() {
            seed::log("request ok!");
        } else {
            seed::log("request nok!");
            let err_msg = response.text().await.unwrap();
            return Msg::RequestNOK(err_msg);
        }
        let employee = response
            .check_status()
            .expect("status check failed")
            .json::<Employee>()
            .await
            .expect("deserialization failed");
        Msg::EmployeeLoaded(employee)
    });
}

fn request_skills(orders: &mut impl Orders<Msg>, id: &str) {
    let url = format!("{}/list_skills_for_employee/{}", BACKEND_ADDRESS, id);
    let request = Request::new(url)
        .method(Method::Get)
        .header(Header::custom("Accept-Language", "en"));

    orders.perform_cmd(async {
        let response = fetch(request).await.expect("HTTP request failed");
        if response.status().is_ok() {
            seed::log("request ok!");
        } else {
            seed::log("request nok!");
            let err_msg = response.text().await.unwrap();
            return Msg::RequestNOK(err_msg);
        }
        let skill_list = response
            .check_status()
            .expect("status check failed")
            .json::<Vec<Skill>>()
            .await
            .expect("deserialization failed");
        Msg::SkillListLoaded(skill_list)
    });
}

pub fn update(
    orders: &mut impl Orders<Msg>,
    model: &mut Model,
    msg: Msg,
) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match msg {
        Msg::EmployeeLoaded(employee) => {
            model.employee = Some(employee);
        },
        Msg::SkillListLoaded(skills) => {
            model.employee_skills = skills;
        },
        Msg::RequestNOK(err_msg) => {
            model.error = Some(err_msg);
        },
        Msg::SearchQueryChanged(query) => {
            model.search_query = query.clone();

            if query.is_empty() {
                model.matched_skills = Vec::<Skill>::new();
                return;
            }

            let url = format!("{}/skill/search/{}", BACKEND_ADDRESS, query);
            let request = Request::new(url)
                .method(Method::Get)
                .header(Header::custom("Accept-Language", "en"));

            orders.perform_cmd(async {
                let response =
                    fetch(request).await.expect("HTTP request failed");
                if response.status().is_ok() {
                    seed::log("request ok!");
                } else {
                    seed::log("request nok!");
                }
                let skill_list = response
                    .check_status()
                    .expect("status check failed")
                    .json::<Vec<Skill>>()
                    .await
                    .expect("deserialization failed");
                Msg::Received(skill_list)
            });
            log!("search query changed 5");
        },
        Msg::Received(skills) => {
            model.matched_skills = skills;
        },
        Msg::AddSkill(skill) => {
            let employee_id = match &model.employee {
                Some(employee) => employee.employee_number.clone(),
                None => unimplemented!(),
            };
            let relation = EmployeeSkill {
                employee_number: employee_id,
                skill_id: skill.id,
            };
            let _request = Request::new(BACKEND_ADDRESS)
                .method(Method::Post)
                .header(Header::custom("Accept-Language", "en"))
                .json(&relation)
                .unwrap(); //TODO: remove unwrap
        },
    }
}

fn list_skills(model: &Model) -> Vec<Node<Msg>> {
    model
        .employee_skills
        .clone()
        .iter()
        .map(
            |skill| ul![
                C![
                    C.text_31,
                    C.relative,
                    C.pl_4,
                    C.pr_4,
                ],
                button![
                    a![
                        attrs!{
                            At::Href => Urls::new(&model.base_url).skill(&skill.id.to_string())
                        },
                        span![
                            skill.skill.clone()
                        ]
                    ]
                ]
            ],
        )
        .collect()
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.error {
        None => employee_found_view(model),
        Some(_) => employee_not_found_view(model),
    }
}

#[allow(clippy::too_many_lines)]
pub fn employee_not_found_view(model: &Model) -> Node<Msg> {
    div![
        C![C.flex_grow,],
        // Main section
        section![
            C![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                C.sm__h_890px,
                C.lg__h_1420px,
            ],
            // Left background
            div![C![C.absolute, C.left_0, C.inset_y_0, C.w_1of2, C.bg_gray_3,]],
            div![
                C![C.relative, C.flex, C.justify_center,],
                // Main container
                div![
                    C![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        C.lg__h_1160px,
                        C.lg__rounded_bl_260px,
                    ],
                    div![
                        C![
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            C.sm__mt_44,
                            C.sm__ml_20,
                            C.sm__w_md,
                            C.lg__ml_20,
                            C.lg__w_216,
                        ],
                        h1![
                            C![
                                C.inline,
                                C.leading_tight,
                                C.text_31,
                                C.text_gray_10
                                C.sm__text_40,
                                C.lg__leading_none,
                                C.lg__text_120,
                            ],
                            span!["Employee not found in the "],
                            span![C![C.font_bold], "Database"],
                        ]
                    ],
                    div![
                        C![
                            C.flex_1,
                            C.w_full,
                            C.mx_auto,
                            C.max_w_sm,
                            C.content_start,
                            C.pt_4,
                            C.mb_6,
                            C.lg__pt_0,
                        ],
                        div![nodes!(list_skills(model))]
                    ]
                ],
            ],
        ],
    ]
}

#[allow(clippy::too_many_lines)]
pub fn employee_found_view(model: &Model) -> Node<Msg> {
    div![
        C![C.flex_grow,],
        // Main section
        section![
            C![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                C.sm__h_890px,
                C.lg__h_1420px,
            ],
            // Left background
            div![C![C.absolute, C.left_0, C.inset_y_0, C.w_1of2, C.bg_gray_3,]],
            div![
                C![C.relative, C.flex, C.justify_center,],
                // Main container
                div![
                    C![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        C.lg__h_1160px,
                        C.lg__rounded_bl_260px,
                    ],
                    div![
                        C![
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            C.sm__mt_44,
                            C.sm__ml_20,
                            C.sm__w_md,
                            C.lg__ml_20,
                            C.lg__w_216,
                        ],
                        match &model.employee {
                            None => div![span!["Loading..."]],
                            Some(employee) => h1![
                                C![
                                    C.inline,
                                    C.leading_tight,
                                    C.text_31,
                                    C.text_gray_10
                                    C.sm__text_40,
                                    C.lg__leading_none,
                                    C.lg__text_120,
                                ],
                                span![C![C.font_bold], &employee.name],
                            ],
                        }
                    ],
                    div![
                        C![
                            C.flex_1,
                            C.w_full,
                            C.mx_auto,
                            C.max_w_sm,
                            C.content_start,
                            C.pt_4,
                            C.mb_6,
                            C.lg__pt_0,
                        ],
                        div![nodes!(list_skills(model))],
                    ],
                    div![
                        normal_text(),
                        span!["Add Skill:"],
                        search_bar(model),
                    ],
                ],
            ],
        ],
    ]
}

fn normal_text() -> Attrs {
    C![
        C.text_31,
        C.flex_1,
        C.w_full,
        C.mx_auto,
        C.max_w_sm,
        C.content_start,
        C.pt_4,
        C.mb_6,
        C.lg__pt_0,
    ]
}

fn search_bar(model: &Model) -> Node<Msg> {
    div![
        C![
            C.flex_1,
            C.w_full,
            C.mx_auto,
            C.max_w_sm,
            C.content_center,
            C.pt_4,
            C.mb_6,
            // lg__
            C.lg__pt_0,
        ],
        div![
            C![
                C.relative, C.pl_4, C.pr_4, // md__
                C.md__pr_0,
            ],
            // search icon
            div![
                C![C.absolute,],
                style! {
                    St::Top => rem(0.6),
                    St::Left => rem(1.5),
                },
            ],
            input![
                C![
                    C.w_full,
                    C.bg_gray_1,
                    C.text_25,
                    C.text_25,
                    IF!(model.search_query.is_empty() => C.font_bold),
                    C.placeholder_gray_4,
                    C.border_b_4,
                    C.border_gray_5,
                    C.focus__outline_none,
                    C.pt_2,
                    C.pb_2,
                    C.px_2,
                    C.pl_8,
                    C.appearance_none,
                ],
                // ev(Ev::KeyPress, |_| Msg::ToggleGuideList),
                attrs! {
                    At::Type => "search",
                    At::Placeholder => "Search",
                    At::Value => model.search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ]
        ],
        div![nodes!(generate_skill_list(model))]
    ]
}

pub fn generate_skill_list(model: &Model) -> Vec<Node<Msg>> {
    seed::log("matched skills:");
    seed::log(model.matched_skills.clone());

    model
        .matched_skills
        .clone()
        .iter()
        .map(
            |skill| {
                let skill = skill.clone();
                ul![
                    C![
                        C.text_25,
                        C.relative, C.pl_4, C.pr_4,
                    ],
                    button![
                        skill.skill.clone(),
                        ev(Ev::Click, move |_| Msg::AddSkill(skill)),
                    ],
                ]
            }
        )
        .collect()
}
