#[cfg(feature = "IoBan")]
use leptos::*;
#[cfg(feature = "IoBan")]
///This icon requires the feature `IoBan` to be enabled.
#[component]
pub fn Ban(
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
        height = size xmlns = "http://www.w3.org/2000/svg" > < circle xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - miterlimit =
        "10" stroke - width = "48" cx = "256" cy = "256" r = "200" />< line xmlns =
        "http://www.w3.org/2000/svg" stroke = "#000" stroke - miterlimit = "10" stroke -
        width = "48" x1 = "114.58" y1 = "114.58" x2 = "397.42" y2 = "397.42" /> < title >
        { title } < / title > < / svg >
    }
}
