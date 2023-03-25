#[cfg(feature = "FiSunrise")]
use leptos::*;
#[cfg(feature = "FiSunrise")]
///This icon requires the feature `FiSunrise` to be enabled.
#[component]
pub fn Sunrise(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 18a5 5 0 0 0-10 0" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12"
        y1 = "2" x2 = "12" y2 = "9" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "4.22" y1 = "10.22" x2 = "5.64" y2 = "11.64" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "1" y1 = "18" x2 = "3" y2 = "18" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "21" y1 = "18" x2 = "23" y2 = "18" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "18.36" y1 = "11.64" x2 = "19.78" y2 =
        "10.22" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "23" y1 = "22" x2 =
        "1" y2 = "22" />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "8 6 12 2 16 6" /> < title > { title } < / title > < / svg >
    }
}
