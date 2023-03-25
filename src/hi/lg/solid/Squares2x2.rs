#[cfg(feature = "HiLgSolidSquares2x2")]
use leptos::*;
#[cfg(feature = "HiLgSolidSquares2x2")]
///This icon requires the feature `HiLgSolidSquares2x2` to be enabled.
#[component]
pub fn Squares2x2(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3 6C3 4.34315 4.34315 3 6 3H8.25C9.90685 3 11.25 4.34315 11.25 6V8.25C11.25 9.90685 9.90685 11.25 8.25 11.25H6C4.34315 11.25 3 9.90685 3 8.25V6ZM12.75 6C12.75 4.34315 14.0931 3 15.75 3H18C19.6569 3 21 4.34315 21 6V8.25C21 9.90685 19.6569 11.25 18 11.25H15.75C14.0931 11.25 12.75 9.90685 12.75 8.25V6ZM3 15.75C3 14.0931 4.34315 12.75 6 12.75H8.25C9.90685 12.75 11.25 14.0931 11.25 15.75V18C11.25 19.6569 9.90685 21 8.25 21H6C4.34315 21 3 19.6569 3 18V15.75ZM12.75 15.75C12.75 14.0931 14.0931 12.75 15.75 12.75H18C19.6569 12.75 21 14.0931 21 15.75V18C21 19.6569 19.6569 21 18 21H15.75C14.0931 21 12.75 19.6569 12.75 18V15.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
