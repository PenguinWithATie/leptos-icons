#[cfg(feature = "VsTextSize")]
use leptos::*;
#[cfg(feature = "VsTextSize")]
///This icon requires the feature `VsTextSize` to be enabled.
#[component]
pub fn TextSize(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.36 7L1 13h1.34l.51-1.47h2.26L5.64 13H7L4.65 7H3.36zm-.15 3.53l.78-2.14.78 2.14H3.21zM11.82 4h-1.6L7 13h1.56l.75-2.29h3.36l.77 2.29H15l-3.18-9zM9.67 9.5l1.18-3.59c.059-.185.1-.376.12-.57.027.192.064.382.11.57l1.25 3.59H9.67z"
        /> < title > { title } < / title > < / svg >
    }
}
