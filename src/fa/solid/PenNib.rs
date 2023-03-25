#[cfg(feature = "FaSolidPenNib")]
use leptos::*;
#[cfg(feature = "FaSolidPenNib")]
///This icon requires the feature `FaSolidPenNib` to be enabled.
#[component]
pub fn PenNib(
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
        "M366.4 18.3L310.7 74.1 435.9 199.3l55.7-55.7c21.9-21.9 21.9-57.3 0-79.2L445.6 18.3c-21.9-21.9-57.3-21.9-79.2 0zM286 94.6l-9.2 2.8L132.7 140.6c-19.9 6-35.7 21.2-42.3 41L1.8 445.8c-3.8 11.3-1 23.9 7.3 32.4L162.7 324.7c-3-6.3-4.7-13.3-4.7-20.7c0-26.5 21.5-48 48-48s48 21.5 48 48s-21.5 48-48 48c-7.4 0-14.4-1.7-20.7-4.7L31.7 500.9c8.6 8.3 21.1 11.2 32.4 7.3l264.3-88.6c19.7-6.6 35-22.4 41-42.3l43.2-144.1 2.8-9.2L286 94.6z"
        /> < title > { title } < / title > < / svg >
    }
}
