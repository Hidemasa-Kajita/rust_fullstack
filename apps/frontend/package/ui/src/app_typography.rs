use stylist::{style, yew::styled_component, Style};
use yew::prelude::{html, Html, Properties};

#[derive(PartialEq, Debug)]
pub enum Tag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Span,
}

impl Tag {
    fn element(&self, value: &str, color_style: Style) -> Html {
        let base_style = style!(
            r#"
                margin: 0;
            "#
        )
        .expect("Failed to mount style");

        match self {
            Tag::H1 => html!(
                <h1
                    class={vec!(
                        style!(r#"
                            font-size: 42px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h1>
            ),
            Tag::H2 => html!(
                <h2
                    class={vec!(
                        style!(r#"
                            font-size: 32px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h2>
            ),
            Tag::H3 => html!(
                <h3
                    class={vec!(
                        style!(r#"
                            font-size: 26px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h3>
            ),
            Tag::H4 => html!(
                <h4
                    class={vec!(
                        style!(r#"
                            font-size: 22px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h4>
            ),
            Tag::H5 => html!(
                <h5
                    class={vec!(
                        style!(r#"
                            font-size: 20px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h5>
            ),
            Tag::H6 => html!(
                <h6
                    class={vec!(
                        style!(r#"
                            font-size: 18px;
                            font-weight: bold;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </h6>
            ),
            Tag::P => html!(
                <p
                    class={vec!(
                        style!(r#"
                            font-size: 16px;
                            font-weight: regular;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </p>
            ),
            Tag::Span => html!(
                <span
                    class={vec!(
                        style!(r#"
                            font-size: 16px;
                        "#).expect("Failed to mount style"),
                        base_style,
                        color_style
                    )}
                >
                    {value}
                </span>
            ),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Black,
    White,
    Red,
    Gray,
}

impl Color {
    fn style(&self) -> Result<Style, stylist::Error> {
        match self {
            Color::Black => style!(
                r#"
                    color: black;
                "#
            ),
            Color::White => style!(
                r#"
                    color: white;
                "#
            ),
            Color::Red => style!(
                r#"
                    color: red;
                "#
            ),
            Color::Gray => style!(
                r#"
                    color: #949497;
                "#
            ),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    #[prop_or(Tag::P)]
    pub tag: Tag,
    #[prop_or(Color::Black)]
    pub color: Color,
}

#[styled_component(AppTypography)]
pub fn app_typography(Props { value, tag, color }: &Props) -> Html {
    let color_style = color.style().unwrap();

    tag.element(value, color_style)
}
