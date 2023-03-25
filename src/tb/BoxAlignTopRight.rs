#[cfg(feature = "TbBoxAlignTopRight")]
use leptos::*;
#[cfg(feature = "TbBoxAlignTopRight")]
///This icon requires the feature `TbBoxAlignTopRight` to be enabled.
#[component]
pub fn BoxAlignTopRight(
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
        "icon icon-tabler icon-tabler-box-align-top-right" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 11.01h-5a1 1 0 0 1 -1 -1v-5a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 15.01v-.01" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M20 20.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 20.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 20.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 4.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 20.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 15.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 9.01v-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 4.01v-.01" /> < title > { title } < / title
        > < / svg >
    }
}
