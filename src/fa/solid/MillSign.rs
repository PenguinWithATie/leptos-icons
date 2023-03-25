#[cfg(feature = "FaSolidMillSign")]
use leptos::*;
#[cfg(feature = "FaSolidMillSign")]
///This icon requires the feature `FaSolidMillSign` to be enabled.
#[component]
pub fn MillSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M302.1 42.8c5.9-16.6-2.7-35-19.4-40.9s-35 2.7-40.9 19.4L208 116.1c-5.7 4-11.1 8.5-16 13.5C171.7 108.9 143.3 96 112 96c-19.5 0-37.8 5-53.7 13.7C52.5 101.4 42.9 96 32 96C14.3 96 0 110.3 0 128v80V416c0 17.7 14.3 32 32 32s32-14.3 32-32V208c0-26.5 21.5-48 48-48s48 21.5 48 48v42.5L81.9 469.2c-5.9 16.6 2.7 35 19.4 40.9s35-2.7 40.9-19.4l21.4-60C168.9 441 179.6 448 192 448c17.7 0 32-14.3 32-32V261.5l35.7-100c3.9-1 8.1-1.6 12.3-1.6c26.5 0 48 21.5 48 48V416c0 17.7 14.3 32 32 32s32-14.3 32-32V208c0-58.2-44.3-106-101.1-111.5l19.2-53.8z"
        /> < title > { title } < / title > < / svg >
    }
}
