#[cfg(feature = "CgLoadbarSound")]
use leptos::*;
#[cfg(feature = "CgLoadbarSound")]
///This icon requires the feature `CgLoadbarSound` to be enabled.
#[component]
pub fn LoadbarSound(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 6H13V18H11V6Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 13H9V18H7V13Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15 9H17V18H15V9Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
