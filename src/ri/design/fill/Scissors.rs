#[cfg(feature = "RiDesignFillScissors")]
use leptos::*;
#[cfg(feature = "RiDesignFillScissors")]
///This icon requires the feature `RiDesignFillScissors` to be enabled.
#[component]
pub fn Scissors(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M9.683 7.562L12 9.88l6.374-6.375a2 2 0 0 1 2.829 0l.707.707L9.683 16.438a4 4 0 1 1-2.121-2.121L9.88 12 7.562 9.683a4 4 0 1 1 2.121-2.121zM6 8a2 2 0 1 0 0-4 2 2 0 0 0 0 4zm0 12a2 2 0 1 0 0-4 2 2 0 0 0 0 4zm9.535-6.587l6.375 6.376-.707.707a2 2 0 0 1-2.829 0l-4.96-4.961 2.12-2.122z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
