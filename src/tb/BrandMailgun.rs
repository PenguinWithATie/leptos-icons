#[cfg(feature = "TbBrandMailgun")]
use leptos::*;
#[cfg(feature = "TbBrandMailgun")]
///This icon requires the feature `TbBrandMailgun` to be enabled.
#[component]
pub fn BrandMailgun(
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
        "icon icon-tabler icon-tabler-brand-mailgun" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 12a2 2 0 1 0 4 0a9 9 0 1 0 -2.987 6.697" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <
        title > { title } < / title > < / svg >
    }
}
