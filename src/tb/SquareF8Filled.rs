#[cfg(feature = "TbSquareF8Filled")]
use leptos::*;
#[cfg(feature = "TbSquareF8Filled")]
///This icon requires the feature `TbSquareF8Filled` to be enabled.
#[component]
pub fn SquareF8Filled(
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
        "icon icon-tabler icon-tabler-square-f8-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.333 3c1.96 0 3.56 1.537 3.662 3.472l.005 .195v10.666c0 1.96 -1.537 3.56 -3.472 3.662l-.195 .005h-10.666a3.667 3.667 0 0 1 -3.662 -3.472l-.005 -.195v-10.666c0 -1.96 1.537 -3.56 3.472 -3.662l.195 -.005h10.666zm-2.333 5h-1l-.15 .005a2 2 0 0 0 -1.844 1.838l-.006 .157v1l.005 .15c.018 .236 .077 .46 .17 .667l.075 .152l.018 .03l-.018 .032c-.133 .24 -.218 .509 -.243 .795l-.007 .174v1l.005 .15a2 2 0 0 0 1.838 1.844l.157 .006h1l.15 -.005a2 2 0 0 0 1.844 -1.838l.006 -.157v-1l-.005 -.15a1.988 1.988 0 0 0 -.17 -.667l-.075 -.152l-.019 -.032l.02 -.03a2.01 2.01 0 0 0 .242 -.795l.007 -.174v-1l-.005 -.15a2 2 0 0 0 -1.838 -1.844l-.157 -.006zm-5 0h-2l-.117 .007a1 1 0 0 0 -.876 .876l-.007 .117v6l.007 .117a1 1 0 0 0 .876 .876l.117 .007l.117 -.007a1 1 0 0 0 .876 -.876l.007 -.117v-2h1l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007h-1v-1h1l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007zm5 5v1h-1v-1h1zm0 -3v1h-1v-1h1z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
