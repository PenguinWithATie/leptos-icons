#[cfg(feature = "TbSquareF2Filled")]
use leptos::*;
#[cfg(feature = "TbSquareF2Filled")]
///This icon requires the feature `TbSquareF2Filled` to be enabled.
#[component]
pub fn SquareF2Filled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-square-f2-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.333 3c1.96 0 3.56 1.537 3.662 3.472l.005 .195v10.666c0 1.96 -1.537 3.56 -3.472 3.662l-.195 .005h-10.666a3.667 3.667 0 0 1 -3.662 -3.472l-.005 -.195v-10.666c0 -1.96 1.537 -3.56 3.472 -3.662l.195 -.005h10.666zm-2.333 5h-2l-.117 .007a1 1 0 0 0 0 1.986l.117 .007h2v1h-1l-.15 .005a2 2 0 0 0 -1.844 1.838l-.006 .157v1l.005 .15a2 2 0 0 0 1.838 1.844l.157 .006h2l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007h-2v-1h1l.15 -.005a2 2 0 0 0 1.844 -1.838l.006 -.157v-1l-.005 -.15a2 2 0 0 0 -1.995 -1.85zm-5 0h-2l-.117 .007a1 1 0 0 0 -.876 .876l-.007 .117v6l.007 .117a1 1 0 0 0 .876 .876l.117 .007l.117 -.007a1 1 0 0 0 .876 -.876l.007 -.117v-2h1l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007h-1v-1h1l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
