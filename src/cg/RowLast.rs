#[cfg(feature = "CgRowLast")]
use leptos::*;
#[cfg(feature = "CgRowLast")]
///This icon requires the feature `CgRowLast` to be enabled.
#[component]
pub fn RowLast(
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
        "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" opacity =
        "0.5" >< path d =
        "M6 13C5.44772 13 5 12.5523 5 12C5 11.4477 5.44772 11 6 11H14C14.5523 11 15 11.4477 15 12C15 12.5523 14.5523 13 14 13H6Z"
        fill = "currentColor" />< path d =
        "M6 9C5.44772 9 5 8.55228 5 8C5 7.44772 5.44772 7 6 7H14C14.5523 7 15 7.44772 15 8C15 8.55228 14.5523 9 14 9H6Z"
        fill = "currentColor" /></ g >< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 16C5 16.5523 5.44772 17 6 17H18C18.5523 17 19 16.5523 19 16C19 15.4477 18.5523 15 18 15H6C5.44772 15 5 15.4477 5 16Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
