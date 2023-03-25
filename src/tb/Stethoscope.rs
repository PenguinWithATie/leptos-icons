#[cfg(feature = "TbStethoscope")]
use leptos::*;
#[cfg(feature = "TbStethoscope")]
///This icon requires the feature `TbStethoscope` to be enabled.
#[component]
pub fn Stethoscope(
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
        "icon icon-tabler icon-tabler-stethoscope" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 4h-1a2 2 0 0 0 -2 2v3.5h0a5.5 5.5 0 0 0 11 0v-3.5a2 2 0 0 0 -2 -2h-1" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8 15a6 6 0 1 0 12 0v-3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M11 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 10m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <
        title > { title } < / title > < / svg >
    }
}
