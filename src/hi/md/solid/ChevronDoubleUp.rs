#[cfg(feature = "HiMdSolidChevronDoubleUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronDoubleUp")]
///This icon requires the feature `HiMdSolidChevronDoubleUp` to be enabled.
#[component]
pub fn ChevronDoubleUp(
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
        "M5.23017 15.7906C4.93159 15.5035 4.92228 15.0287 5.20938 14.7302L9.45938 10.2302C9.60078 10.0831 9.79599 10 10 10C10.204 10 10.3992 10.0831 10.5406 10.2302L14.7906 14.7302C15.0777 15.0287 15.0684 15.5035 14.7698 15.7906C14.4713 16.0777 13.9965 16.0684 13.7094 15.7698L10 11.8321L6.29063 15.7698C6.00353 16.0684 5.52875 16.0777 5.23017 15.7906ZM5.23017 9.79062C4.93159 9.50353 4.92228 9.02875 5.20938 8.73017L9.45938 4.23017C9.60078 4.08311 9.79599 4 10 4C10.204 4 10.3992 4.08311 10.5406 4.23017L14.7906 8.73017C15.0777 9.02875 15.0684 9.50353 14.7698 9.79062C14.4713 10.0777 13.9965 10.0684 13.7094 9.76983L10 5.83208L6.29063 9.76983C6.00353 10.0684 5.52875 10.0777 5.23017 9.79062Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
