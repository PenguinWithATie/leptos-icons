#[cfg(feature = "TbKeyboard")]
use leptos::*;
#[cfg(feature = "TbKeyboard")]
///This icon requires the feature `TbKeyboard` to be enabled.
#[component]
pub fn Keyboard(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-keyboard"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 6m0 2a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 14l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 14l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 14l4 .01" /> < title > { title } < / title
        > < / svg >
    }
}
