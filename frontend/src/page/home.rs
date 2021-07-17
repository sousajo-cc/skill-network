use crate::page::*;

pub struct Model {
    pub base_url: Url,
    pub search_bar: SearchBar<Skill>,
}

impl Model {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            search_bar: SearchBar::new(),
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    SearchQueryChanged(String),
    Received(Vec<Skill>),
}

pub fn init(mut _orders: impl Orders<Msg>) {
    document().set_title(TITLE_SUFFIX);
}

pub fn generate_skill_list(model: &Model) -> Vec<Node<Msg>> {
    model
        .search_bar
        .matched_skills
        .clone()
        .iter()
        .map(
            |skill| ul![
                C![
                    C.text_25,
                    C.relative, C.pl_4, C.pr_4,
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

pub fn update(orders: &mut impl Orders<Msg>, model: &mut Model, msg: Msg) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match msg {
        Msg::SearchQueryChanged(query) => {
            if query.is_empty() {
                model.search_bar.matched_skills = Vec::<Skill>::new();
                return;
            }

            let url = format!("{}/skill/search/{}", BACKEND_ADDRESS, query);
            let request = Request::new(url)
                .method(Method::Get)
                .header(Header::custom("Accept-Language", "en"));

            model.search_bar.search_query = query;

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
            model.search_bar.matched_skills = skills;
        },
    }
}

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![C.flex_grow,],
        // Main section
        section![
            C![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                // sm__
                C.sm__h_890px,
                // lg__
                C.lg__h_1420px,
            ],
            // Left background
            div![C![
                C.absolute,
                C.left_0,
                C.inset_y_0,
                C.w_1of2,
                C.bg_gray_3,
                // C.bg_yellow_4,
            ]],
            div![
                C![C.relative, C.flex, C.justify_center,],
                // Main container
                div![
                    C![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        // sm__
                        // C.sm__h_550px,
                        // lg__
                        C.lg__h_1160px,
                        C.lg__rounded_bl_260px,
                    ],
                    div![
                        C![
                            // C.mt_40,
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            // sm__
                            // sousajo sm__mt_44 changes the position of the text in the main page
                            C.sm__mt_44,
                            C.sm__ml_20,
                            C.sm__w_md,
                            // lg__
                            // C.lg__mt_72,
                            C.lg__ml_20,
                            C.lg__w_216,
                        ],
                        h1![
                            C![
                                C.inline,
                                C.leading_tight,
                                C.text_31,
                                C.text_gray_10
                                // sm__
                                C.sm__text_40,
                                // lg__
                                C.lg__leading_none,
                                C.lg__text_120,
                            ],
                            span!["I need someone with "],
                            span![C![C.font_bold], "Skill"],
                        ],
                        span![
                            C![
                                C.text_21,
                                C.text_gray_7,
                                // sm__
                                C.sm__text_32,
                                // lg__
                                C.lg__text_60,
                            ],
                            "\u{00A0}in",
                            br![],
                            "..."
                        ]
                    ],
                    search_bar(model)
                ],
            ],
            // Gear
        ],
        // Circles
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
                    IF!(model.search_bar.search_query.is_empty() => C.font_bold),
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
                    At::Value => model.search_bar.search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ]
        ],
        div![nodes!(generate_skill_list(model))]
    ]
}