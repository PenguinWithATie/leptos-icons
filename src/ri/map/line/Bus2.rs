#[cfg(feature = "RiMapLineBus2")]
use leptos::*;
#[cfg(feature = "RiMapLineBus2")]
///This icon requires the feature `RiMapLineBus2` to be enabled.
#[component]
pub fn Bus2(
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
        "M17 20H7v1a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-9H2V8h1V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3h1v4h-1v9a1 1 0 0 1-1 1h-2a1 1 0 0 1-1-1v-1zM5 5v6h14V5H5zm14 8H5v5h14v-5zM7.5 17a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm9 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
