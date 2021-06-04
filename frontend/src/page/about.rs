use crate::page::*;

#[derive(Debug)]
pub enum Msg {}

#[allow(clippy::too_many_lines)]
pub fn view() -> Node<Msg> {
    div![
        C![
            C.flex_grow,
        ],
/*        section![
            C![
                C.w_screen,
                C.h_690px,
                C.bg_blue_10,
                // sm__
                C.sm__h_790px,
                // lg__
                C.lg__h_1420px,
            ],
            div![
                C![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.h_320px,
                    C.rounded_bl_140px,
                    C.bg_gray_1,
                    // sm__
                    C.sm__h_420px,
                    // lg__
                    C.lg__h_600px,
                    C.lg__rounded_bl_330px,
                ],
            ],
        ],*/

        // Resume section
        section![
            C![
                C.flex,
                C.flex_col,
                C.justify_center,
                C.items_center,
            ],
            a![
                attrs!{
                    At::Href => "https://p"
                },
                C![
                    C.mt_16,
                    C.mb_20,
                    C.flex,
                    C.items_center,
                    C.justify_center,
                    C.text_19,
                    C.text_gray_10,
                    C.md__hover__text_yellow_7,
                    // sm__
                    C.sm__mt_24,
                    C.sm__mb_32,
                    C.sm__text_29,
                    // lg__
                    C.lg__mt_40,
                    C.lg__mb_40
                    C.lg__text_45,
                ],
                "Go to our\u{00A0}",
                span![
                    C![
                        C.font_semibold
                    ],
                    "GitHub"
                ],
                "\u{00A0}profile",
                img![
                    C![
                        C._mt_4,
                        C.w_4,
                        // sm__
                        C.sm__w_5,
                        // lg__
                        C.lg___mt_10,
                        C.lg__w_8,
                    ],
                    attrs!{
                        At::Src => image_src("link_arrow.svg")
                    }
                ],
            ]
        ],
    ]
}
