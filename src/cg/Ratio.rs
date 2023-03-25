#[cfg(feature = "CgRatio")]
use leptos::*;
#[cfg(feature = "CgRatio")]
///This icon requires the feature `CgRatio` to be enabled.
#[component]
pub fn Ratio(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 6V12H6V8L10 8V6H4Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 18H14V16H18V12H20V18Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 2C1.79086 2 0 3.79086 0 6V18C0 20.2091 1.79086 22 4 22H20C22.2091 22 24 20.2091 24 18V6C24 3.79086 22.2091 2 20 2H4ZM20 4H4C2.89543 4 2 4.89543 2 6V18C2 19.1046 2.89543 20 4 20H20C21.1046 20 22 19.1046 22 18V6C22 4.89543 21.1046 4 20 4Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
