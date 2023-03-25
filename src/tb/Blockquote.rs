#[cfg(feature = "TbBlockquote")]
use leptos::*;
#[cfg(feature = "TbBlockquote")]
///This icon requires the feature `TbBlockquote` to be enabled.
#[component]
pub fn Blockquote(
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
        "icon icon-tabler icon-tabler-blockquote" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 15h15" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 19h-15" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 11h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 7h-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 9h1a1 1 0 1 1 -1 1v-2.5a2 2 0 0 1 2 -2" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 9h1a1 1 0 1 1 -1 1v-2.5a2 2 0 0 1 2 -2" /> < title > { title } < / title > <
        / svg >
    }
}
