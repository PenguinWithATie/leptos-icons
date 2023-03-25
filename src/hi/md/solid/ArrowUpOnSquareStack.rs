#[cfg(feature = "HiMdSolidArrowUpOnSquareStack")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUpOnSquareStack")]
///This icon requires the feature `HiMdSolidArrowUpOnSquareStack` to be enabled.
#[component]
pub fn ArrowUpOnSquareStack(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.75 6L8.75 6V10.25C8.75 10.6642 8.41421 11 8 11C7.58579 11 7.25 10.6642 7.25 10.25V6L8.74999 6V3.70447L9.69252 4.75172C9.96962 5.05961 10.4438 5.08456 10.7517 4.80747C11.0596 4.53038 11.0846 4.05616 10.8075 3.74828L8.55747 1.24828C8.41523 1.09024 8.21261 1 7.99999 1C7.78738 1 7.58476 1.09024 7.44252 1.24828L5.19252 3.74828C4.91543 4.05616 4.94039 4.53038 5.24827 4.80747C5.55615 5.08456 6.03037 5.05961 6.30746 4.75172L7.24999 3.70447V6H5.25C4.00736 6 3 7.00736 3 8.25V12.75C3 13.9926 4.00736 15 5.25 15H10.75C11.9926 15 13 13.9926 13 12.75V8.25C13 7.00736 11.9926 6 10.75 6ZM7 16.75V16.5H10.75C12.8211 16.5 14.5 14.8211 14.5 12.75V10H14.75C15.9926 10 17 11.0074 17 12.25V16.75C17 17.9926 15.9926 19 14.75 19H9.25C8.00736 19 7 17.9926 7 16.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
