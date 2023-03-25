#[cfg(feature = "FaSolidArrowDownAZ")]
use leptos::*;
#[cfg(feature = "FaSolidArrowDownAZ")]
///This icon requires the feature `FaSolidArrowDownAZ` to be enabled.
#[component]
pub fn ArrowDownAZ(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M143.6 469.6C137.5 476.2 129 480 120 480s-17.5-3.8-23.6-10.4l-88-96c-11.9-13-11.1-33.3 2-45.2s33.3-11.1 45.2 2L88 365.7V64c0-17.7 14.3-32 32-32s32 14.3 32 32V365.7l32.4-35.4c11.9-13 32.2-13.9 45.2-2s13.9 32.2 2 45.2l-88 96zM280 320c0-17.7 14.3-32 32-32H440c12.9 0 24.6 7.8 29.6 19.8s2.2 25.7-6.9 34.9L389.3 416H440c17.7 0 32 14.3 32 32s-14.3 32-32 32H312c-12.9 0-24.6-7.8-29.6-19.8s-2.2-25.7 6.9-34.9L362.7 352H312c-17.7 0-32-14.3-32-32zM376 32c12.1 0 23.2 6.8 28.6 17.7l64 128 16 32c7.9 15.8 1.5 35-14.3 42.9s-35 1.5-42.9-14.3L420.2 224H331.8l-7.2 14.3c-7.9 15.8-27.1 22.2-42.9 14.3s-22.2-27.1-14.3-42.9l16-32 64-128C352.8 38.8 363.9 32 376 32zM355.8 176h40.4L376 135.6 355.8 176z"
        /> < title > { title } < / title > < / svg >
    }
}
