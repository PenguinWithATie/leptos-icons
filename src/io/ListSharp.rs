#[cfg(feature = "IoListSharp")]
use leptos::*;
#[cfg(feature = "IoListSharp")]
///This icon requires the feature `IoListSharp` to be enabled.
#[component]
pub fn ListSharp(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "144" y1 = "144" x2 = "464" y2 = "144" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:48px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "144" y1 = "256" x2 = "464" y2 = "256" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:48px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "144" y1 = "368" x2 = "464" y2 = "368" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:48px" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "64" y = "128" width = "32" height = "32" style
        =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "64" y = "240" width = "32"
        height = "32" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "64" y = "352" width = "32"
        height = "32" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
