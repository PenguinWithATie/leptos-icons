#[cfg(feature = "CgDice5")]
use leptos::*;
#[cfg(feature = "CgDice5")]
///This icon requires the feature `CgDice5` to be enabled.
#[component]
pub fn Dice5(
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
        "M14.9451 7.05518C14.9451 5.95061 15.8405 5.05518 16.9451 5.05518C18.0496 5.05518 18.9451 5.95061 18.9451 7.05518C18.9451 8.15975 18.0496 9.05518 16.9451 9.05518C15.8405 9.05518 14.9451 8.15975 14.9451 7.05518Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.9451 14.8921C15.8405 14.8921 14.9451 15.7875 14.9451 16.8921C14.9451 17.9967 15.8405 18.8921 16.9451 18.8921C18.0496 18.8921 18.9451 17.9967 18.9451 16.8921C18.9451 15.7875 18.0496 14.8921 16.9451 14.8921Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.05518 16.8921C5.05518 15.7875 5.95061 14.8921 7.05518 14.8921C8.15975 14.8921 9.05518 15.7875 9.05518 16.8921C9.05518 17.9967 8.15975 18.8921 7.05518 18.8921C5.95061 18.8921 5.05518 17.9967 5.05518 16.8921Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.05518 5.05518C5.95061 5.05518 5.05518 5.95061 5.05518 7.05518C5.05518 8.15975 5.95061 9.05518 7.05518 9.05518C8.15975 9.05518 9.05518 8.15975 9.05518 7.05518C9.05518 5.95061 8.15975 5.05518 7.05518 5.05518Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 12C10 10.8954 10.8954 10 12 10C13.1046 10 14 10.8954 14 12C14 13.1046 13.1046 14 12 14C10.8954 14 10 13.1046 10 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 4C1 2.34315 2.34315 1 4 1H20C21.6569 1 23 2.34315 23 4V20C23 21.6569 21.6569 23 20 23H4C2.34315 23 1 21.6569 1 20V4ZM4 3H20C20.5523 3 21 3.44772 21 4V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
