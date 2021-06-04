use crate::page::*;

#[derive(Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ScrollToTop,
    Scrolled,
    ToggleMenu,
    HideMenu,
    Received(Vec<Employee>),
}

pub fn init(mut orders: impl Orders<Msg>, id: &String) {
    document().set_title(id);
    request_employees(&mut orders, id);
}

fn request_employees(orders: &mut impl Orders<Msg>, id: &String) {
    let url = format!("http://localhost:8000/list_employees_with_skill/{}", id);
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
        Msg::Received(employee_list)
    });
}

pub fn update(_orders: &mut impl Orders<Msg>, model: &mut Model, msg: Msg) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::new(url);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled => {
            let mut position = body().scroll_top();
            if position == 0 {
                position = document()
                    .document_element()
                    .expect("get document element")
                    .scroll_top()
            }
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Visibility::Hidden;
        },
        Msg::Received(employees) => {
            model.matched_employees = employees;
        }
    }
}

fn list_employees(model: &Model, id: &String) -> Vec<Node<Msg>> {
    seed::log!("employees:", id);
    seed::log(&model.matched_employees);

    model
        .matched_employees
        .clone()
        .iter()
        .map(
            |employee| ul![
                C![
                    C.text_25,
                    C.relative, C.pl_4, C.pr_4,
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
pub fn view(model: &Model, id: &String) -> Node<Msg> {
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
                                    // C.font_bold,
                                    C.text_25,
                                    C.text_25,
                                    IF!(model.search_query.is_empty() => C.font_bold),
                                    // IF!(not(model.in_prerendering) => C.placeholder_green_800),
                                    // IF!(model.in_prerendering => C.placeholder_green_400),
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
                            ]
                        ],
                        div![nodes!(list_employees(model, id))]
                    ]
                ],
            ],
            // Gear
        ],
        // Circles
    ]
}
