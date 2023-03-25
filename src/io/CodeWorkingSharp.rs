#[cfg(feature = "IoCodeWorkingSharp")]
use leptos::*;
#[cfg(feature = "IoCodeWorkingSharp")]
///This icon requires the feature `IoCodeWorkingSharp` to be enabled.
#[component]
pub fn CodeWorkingSharp(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "26" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:10px" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "346" cy = "256" r = "26" style
        = "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:10px" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "166" cy = "256" r = "26" style
        = "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:10px" /><
        polyline xmlns = "http://www.w3.org/2000/svg" points = "160 368 32 256 160 144"
        style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:42px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "352 368 480 256 352 144" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:42px"
        /> < title > { title } < / title > < / svg >
    }
}
