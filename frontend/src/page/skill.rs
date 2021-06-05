use crate::page::*;

#[derive(Debug)]
pub enum Msg {
    SkillLoaded(Skill),
    EmployeeListLoaded(Vec<Employee>),
}

pub fn init(mut orders: impl Orders<Msg>, id: &String) {
    document().set_title(id);
    scroll_to_top();
    request_skill(&mut orders, id);
    request_employees(&mut orders, id);
}

fn request_skill(orders: &mut impl Orders<Msg>, id: &String) {
    let url = format!("{}/skill/{}", BACKEND_ADDRESS, id);
    let request = Request::new(url)
        .method(Method::Get)
        .header(Header::custom("Accept-Language", "en"));

    orders.perform_cmd(async {
        let response = fetch(request).await.expect("HTTP request failed");
        if response.status().is_ok() {
            seed::log("request ok!");
        } else {
            seed::log("request nok!");
        }
        let skill = response
            .check_status()
            .expect("status check failed")
            .json::<Skill>()
            .await
            .expect("deserialization failed");
        Msg::SkillLoaded(skill)
    });
}

fn request_employees(orders: &mut impl Orders<Msg>, id: &String) {
    let url = format!("{}/list_employees_with_skill/{}", BACKEND_ADDRESS, id);
    let request = Request::new(url)
        .method(Method::Get)
        .header(Header::custom("Accept-Language", "en"));

    orders.perform_cmd(async {
        let response = fetch(request).await.expect("HTTP request failed");
        if response.status().is_ok() {
            seed::log("request ok!");
        } else {
            seed::log("request nok!");
        }
        let employee_list = response
            .check_status()
            .expect("status check failed")
            .json::<Vec<Employee>>()
            .await
            .expect("deserialization failed");
        Msg::EmployeeListLoaded(employee_list)
    });
}

pub fn update(_orders: &mut impl Orders<Msg>, model: &mut Model, msg: Msg) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match msg {
        Msg::SkillLoaded(skill) => {
            model.skill = Some(skill);
        }
        Msg::EmployeeListLoaded(employees) => {
            model.matched_employees = employees;
        }
    }
}

fn list_employees(model: &Model) -> Vec<Node<Msg>> {
    model
        .matched_employees
        .clone()
        .iter()
        .map(
            |employee| ul![
                C![
                    C.text_31,
                    C.relative,
                    C.pl_4,
                    C.pr_4,
                ],
                button![
                    a![
                        attrs!{
                            At::Href => Urls::new(&model.base_url).about()
                        },
                        span![
                            employee.name.clone()
                        ]
                    ]
                ]
            ],
        )
        .collect()
}

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> Node<Msg> {
    if !model.matched_employees.is_empty() {
        seed::log(&model.matched_employees[0].name);
    }
    match &model.skill {
        None =>
            div![
                span!["Not Found"]
            ],
        Some(skill) =>
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
                    div![C![
                        C.absolute,
                        C.left_0,
                        C.inset_y_0,
                        C.w_1of2,
                        C.bg_gray_3,
                    ]],
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
                                    span!["People that know "],
                                    span![C![C.font_bold], &skill.skill],
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
                                div![nodes!(list_employees(model))]
                            ]
                        ],
                    ],
                ],
            ]
    }
}
