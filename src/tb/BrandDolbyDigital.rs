#[cfg(feature = "TbBrandDolbyDigital")]
use leptos::*;
#[cfg(feature = "TbBrandDolbyDigital")]
///This icon requires the feature `TbBrandDolbyDigital` to be enabled.
#[component]
pub fn BrandDolbyDigital(
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
        "icon icon-tabler icon-tabler-brand-dolby-digital" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 6v12h-.89c-3.34 0 -6.047 -2.686 -6.047 -6s2.707 -6 6.046 -6h.891z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.063 6v12h.891c3.34 0 6.046 -2.686 6.046 -6s-2.707 -6 -6.046 -6h-.89z" /> <
        title > { title } < / title > < / svg >
    }
}
