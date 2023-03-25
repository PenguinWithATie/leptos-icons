#[cfg(feature = "HiMdSolidChevronDoubleLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronDoubleLeft")]
///This icon requires the feature `HiMdSolidChevronDoubleLeft` to be enabled.
#[component]
pub fn ChevronDoubleLeft(
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
        "M15.7906 14.7698C15.5035 15.0684 15.0287 15.0777 14.7302 14.7906L10.2302 10.5406C10.0831 10.3992 10 10.204 10 10C10 9.79599 10.0831 9.60078 10.2302 9.45938L14.7302 5.20937C15.0287 4.92228 15.5035 4.93159 15.7906 5.23017C16.0777 5.52875 16.0684 6.00353 15.7698 6.29062L11.8321 10L15.7698 13.7094C16.0684 13.9965 16.0777 14.4713 15.7906 14.7698ZM9.79062 14.7698C9.50353 15.0684 9.02875 15.0777 8.73017 14.7906L4.23017 10.5406C4.08311 10.3992 4 10.204 4 10C4 9.79599 4.08311 9.60078 4.23017 9.45938L8.73017 5.20938C9.02875 4.92228 9.50353 4.93159 9.79062 5.23017C10.0777 5.52875 10.0684 6.00353 9.76983 6.29062L5.83208 10L9.76983 13.7094C10.0684 13.9965 10.0777 14.4713 9.79062 14.7698Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
