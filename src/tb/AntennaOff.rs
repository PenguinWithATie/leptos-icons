#[cfg(feature = "TbAntennaOff")]
use leptos::*;
#[cfg(feature = "TbAntennaOff")]
///This icon requires the feature `TbAntennaOff` to be enabled.
#[component]
pub fn AntennaOff(
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
        "icon icon-tabler icon-tabler-antenna-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 4v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4.5v7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5v3m0 4v9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 8v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 6v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8h-8m-4 0h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
