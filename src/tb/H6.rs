#[cfg(feature = "TbH6")]
use leptos::*;
#[cfg(feature = "TbH6")]
///This icon requires the feature `TbH6` to be enabled.
#[component]
pub fn H6(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-h-6" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 14a2 2 0 1 0 0 4a2 2 0 0 0 0 -4z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12a2 2 0 1 0 -4 0v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 6v12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6v12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 18h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 18h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 6h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 6h2" /> < title > { title } < / title > < /
        svg >
    }
}
