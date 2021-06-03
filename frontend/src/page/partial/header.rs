 use crate::{
    generated::css_classes::C,
    image_src, Page, ScrollHistory, Urls,
};
use seed::{prelude::*, *};
use crate::page::home::Model;
use crate::page::common::visibility::Visibility;

 #[derive(Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ScrollToTop,
    Scrolled,
    ToggleMenu,
    SearchQueryChanged(String),
    HideMenu,
}

fn header_visibility(
    menu_visibility: Visibility,
    scroll_history: &ScrollHistory,
) -> Visibility {
    let menu_is_visible = menu_visibility == Visibility::Visible;
    // You can go higher on the mobile phones.
    let at_the_top_or_higher = *scroll_history.back().unwrap_or(&0) <= 0;
    let scrolling_up = scroll_history.front() >= scroll_history.back();

    if menu_is_visible || at_the_top_or_higher || scrolling_up {
        return Visibility::Visible;
    }
    Visibility::Hidden
}

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let show_header =
        header_visibility(model.menu_visibility, &model.scroll_history)
            == Visibility::Visible;
    nodes![
        // Header background and line container
        IF!(show_header =>
            div![
                C![
                    C.fixed,
                    C.top_0
                    C.inset_x_0,
                    C.h_16,
                    // sm__
                    C.sm__h_24,
                ],
                // Header background
                div![C![C.absolute, C.inset_0, C.bg_gray_2, C.opacity_90]],

            ]
        ),
        // Photo 1
        IF!(model.page == Page::About =>
            div![
                C![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.mt_6,
                    C.flex,
                    C.justify_center
                    // sm__
                    C.sm__mt_10,
                    // md__
                    C.md__mt_8,
                ],
            ]
        ),
        // Menu
        IF!(model.menu_visibility == Visibility::Visible =>
            div![
                C![
                    C.fixed,
                    C.w_screen,
                    C.bg_gray_1,
                    C.opacity_90,
                    C.h_screen,
                    // md__
                    C.md__hidden,
                ],
                div![
                    C![C.w_56, C.mx_auto, C.flex, C.max_h_full,],
                    ul![
                        C![
                            C.mt_20,
                            C.w_full,
                            C.font_semibold,
                            C.text_gray_10,
                            C.flex,
                            C.flex_col,
                            C.mb_12,
                            C.overflow_y_auto,
                            // sm__
                            C.sm__mt_24,
                            C.sm__text_21,
                        ],
                        li![
                            C![
                                C.block,
                                C.h_full,
                                C.border_l_4,
                                C.border_r_4,
                                if model.page == Page::Home {
                                    C.border_yellow_6
                                } else {
                                    C.border_gray_2
                                },
                                C.w_full,
                                // sm__
                                C.sm__hidden,
                            ],
                            a![
                                C![
                                    C.pl_8,
                                    C.h_full,
                                    C.flex,
                                    C.items_center,
                                    C.hover__text_yellow_7,
                                    C.outline_none,
                                    C.py_6,
                                ],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).home()
                                },
                                ev(Ev::Click, |_| Msg::ScrollToTop),
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "Update your skillset"
                            ]
                        ],
                        li![
                            C![C.block, C.h_full, C.w_full,],
                            a![
                                C![
                                    C.pl_8,
                                    C.h_full,
                                    C.flex,
                                    C.items_center,
                                    C.hover__text_yellow_7,
                                    C.py_6,
                                    // sm__
                                    C.sm__py_8,
                                    C.sm__pl_0,
                                    C.sm__justify_center,
                                ],
                                attrs! {
                                    At::Href => "https://github.com/sousajf1"
                                },
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "GitHub",
                                img![
                                    C![
                                        C.inline
                                        C.mb_3,
                                        C.w_3,
                                        // sm__
                                        C.sm__mb_5,
                                        C.sm__ml_px,
                                        C.sm__w_4,
                                    ],
                                    attrs! {
                                        At::Src => image_src("link_arrow.svg")
                                    }
                                ]
                            ]
                        ],
                    ],
                ]
            ]
        ),
        // Header
        IF!(show_header =>
            header![
                C![
                    C.fixed,
                    C.top_0
                    C.inset_x_0,
                ],
                // Header controls container
                div![
                    C![
                        C.mx_8
                        C.h_16,
                        C.flex,
                        C.justify_between,
                        C.items_center,
                        // sm__
                        C.sm__h_24,
                    ],
                    // Logo
                    a![
                        attrs! {
                            At::Href => Urls::new(&model.base_url).home()
                        },
                        img![
                            C![
                                C.h_6,
                                // sm__
                                C.sm__h_10,
                                C.sm__w_265px,
                            ],
                            attrs! {
                                At::Src => image_src("logo.svg")
                            }
                        ],
                    ],
                    // Links
                    ul![
                        C![
                            C.hidden,
                            // sm__
                            C.sm___mt_px,
                            C.sm__text_21,
                            C.sm__font_semibold,
                            C.sm__text_gray_10,
                            C.sm__flex,
                            C.sm__items_center,
                            C.sm__h_full,
                        ],
                        li![
                            C![
                                // sm__
                                C.sm__block,
                                C.sm__h_full,
                            ],
                            a![
                                C![
                                    // sm__
                                    C.sm__h_full,
                                    C.sm__flex,
                                    C.sm__items_center,
                                    C.sm__hover__text_yellow_7,
                                    C.sm__outline_none,
                                ],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).home()
                                },
                                "Update your skillset"
                            ]
                        ],
                        li![
                            C![
                                C.hidden,
                                // md__
                                C.md__block,
                                C.md__ml_8,
                                C.md__h_full,
                            ],
                            a![
                                C![
                                    // md__
                                    C.md__h_full,
                                    C.md__flex,
                                    C.md__items_center,
                                    C.md__hover__text_yellow_7,
                                ],
                                attrs! {
                                    At::Href => "https://github.com/sousajf1"
                                },
                                "GitHub",
                                img![
                                    C![
                                        // md__
                                        C.md__inline
                                        C.md__mb_5,
                                        C.md__ml_px,
                                        C.md__w_4,
                                    ],
                                    attrs! {
                                        At::Src => image_src("link_arrow.svg")
                                    }
                                ]
                            ]
                        ],
                    ],
                    // Hamburger
                    div![
                        C![
                            IF!(not(model.in_prerendering) => C.cursor_pointer),
                            // md__
                            C.md__hidden,
                        ],
                        ev(Ev::Click, |_| Msg::ToggleMenu),
                        img![
                            id!("hamburger"),
                            C![
                                C.h_8,
                                C.w_12,
                                // sm__
                                C.sm__h_10
                                C.sm__w_16,
                            ],
                            if model.in_prerendering {
                                attrs! {
                                    At::Src => image_src("loading.svg")
                                }
                            } else {
                                attrs! {
                                    At::Src => if model.menu_visibility == Visibility::Visible {
                                        image_src("cross.svg")
                                    } else {
                                        image_src("hamburger.svg")
                                    }
                                }
                            }
                        ]
                    ],
                    // Spacer
                    div![C![
                        C.hidden,
                        // md__
                        C.md__block,
                    ],],
                ],
                // Bottom header line
            ]
        ),
    ]
}
