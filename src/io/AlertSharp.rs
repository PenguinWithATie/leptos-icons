#[cfg(feature = "IoAlertSharp")]
use leptos::*;
#[cfg(feature = "IoAlertSharp")]
///This icon requires the feature `IoAlertSharp` to be enabled.
#[component]
pub fn AlertSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points = "240 80 248 320 264 320 272 80 240 80" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "240" y = "400" width = "32"
        height = "32" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
