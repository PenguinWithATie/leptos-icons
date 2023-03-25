#[cfg(feature = "TbBrandHbo")]
use leptos::*;
#[cfg(feature = "TbBrandHbo")]
///This icon requires the feature `TbBrandHbo` to be enabled.
#[component]
pub fn BrandHbo(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-brand-hbo"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M2 16v-8" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M6 8v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 8a4 4 0 1 1 0 8a4 4 0 0 1 0 -8z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> < title > { title } < / title > < /
        svg >
    }
}
