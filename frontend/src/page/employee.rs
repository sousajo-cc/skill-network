use crate::page::*;

#[derive(Debug)]
pub enum Msg {
    SkillListLoaded(Vec<Skill>),
    RequestNOK(String),
    //employename
}

pub fn init(mut orders: impl Orders<Msg>, employee_number: &str) {
    document().set_title("Employee Details");
    scroll_to_top();
}


pub fn update(_orders: &mut impl Orders<Msg>, model: &mut Model, msg: Msg) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match msg {
        Msg::SkillListLoaded(skills) => {
            model.employee_skills = skills;
        }
        Msg::RequestNOK(err_msg) => {
            model.employee_error = Some(err_msg);
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
pub fn view(model: &Model) -> Node<Msg> {
    match &model.error {
        None => skill_found_view(model),
        Some(_) =>skill_not_found_view(model),
    }
}

#[allow(clippy::too_many_lines)]
pub fn skill_not_found_view(model: &Model) -> Node<Msg> {
    if !model.matched_employees.is_empty() {
        seed::log(&model.matched_employees[0].name);
    }

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
                            span!["Skill not found in the "],
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
                        div![nodes!(list_employees(model))]
                    ]
                ],
            ],
        ],
    ]

}

#[allow(clippy::too_many_lines)]
pub fn skill_found_view(model: &Model) -> Node<Msg> {
    if !model.matched_employees.is_empty() {
        seed::log(&model.matched_employees[0].name);
    }

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
                        match &model.skill {
                            None =>
                                div![
                                    span!["Loading..."]
                                ],
                            Some(skill) =>
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
                        div![nodes!(list_employees(model))]
                    ]
                ],
            ],
        ],
    ]

}