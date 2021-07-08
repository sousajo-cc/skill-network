use crate::page::*;

pub struct Model;

#[derive(Debug)]
pub enum Msg {}

pub fn init(mut _orders: impl Orders<Msg>) {
    document().set_title(&format!("About - {}", TITLE_SUFFIX));
}

#[allow(clippy::too_many_lines)]
pub fn view(_: &Model) -> Node<Msg> {
    div![
        C![C.flex_grow,],
        // Resume section
        section![
            C![C.flex, C.flex_col, C.justify_center, C.items_center,],
            a![
                attrs! {
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
                span![C![C.font_semibold], "GitHub"],
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
                    attrs! {
                        At::Src => image_src("link_arrow.svg")
                    }
                ],
            ]
        ],
    ]
}
