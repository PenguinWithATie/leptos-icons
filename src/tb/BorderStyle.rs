#[cfg(feature = "TbBorderStyle")]
use leptos::*;
#[cfg(feature = "TbBorderStyle")]
///This icon requires the feature `TbBorderStyle` to be enabled.
#[component]
pub fn BorderStyle(
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
        "icon icon-tabler icon-tabler-border-style" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 20v-14a2 2 0 0 1 2 -2h14"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 8v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 20v.01" /> < title > { title } < / title >
        < / svg >
    }
}
