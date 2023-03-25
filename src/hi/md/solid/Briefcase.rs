#[cfg(feature = "HiMdSolidBriefcase")]
use leptos::*;
#[cfg(feature = "HiMdSolidBriefcase")]
///This icon requires the feature `HiMdSolidBriefcase` to be enabled.
#[component]
pub fn Briefcase(
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
        "M6 3.75C6 2.23122 7.23122 1 8.75 1H11.25C12.7688 1 14 2.23122 14 3.75V4.19269C14.572 4.24808 15.1407 4.31524 15.7057 4.39392C17.0526 4.58149 18 5.74901 18 7.07023V10.5386C18 11.6653 17.3058 12.7301 16.1705 13.0786C14.2185 13.6778 12.1462 14 10 14C7.8538 14 5.78149 13.6778 3.82951 13.0786C2.69423 12.7301 2 11.6653 2 10.5386V7.07023C2 5.74901 2.94737 4.58149 4.29435 4.39392C4.85933 4.31524 5.42796 4.24808 6 4.19269V3.75ZM12.5 3.75V4.07499C11.673 4.02523 10.8394 4 10 4C9.16061 4 8.32704 4.02523 7.5 4.07499V3.75C7.5 3.05964 8.05964 2.5 8.75 2.5H11.25C11.9404 2.5 12.5 3.05964 12.5 3.75ZM10 10C9.44772 10 9 10.4477 9 11V11.01C9 11.5623 9.44772 12.01 10 12.01H10.01C10.5623 12.01 11.01 11.5623 11.01 11.01V11C11.01 10.4477 10.5623 10 10.01 10H10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 15.0552V14.3714C3.1256 14.4243 3.25542 14.4715 3.38933 14.5126C5.48234 15.1551 7.70295 15.5 10 15.5C12.297 15.5 14.5177 15.1551 16.6107 14.5126C16.7446 14.4715 16.8744 14.4243 17 14.3714V15.0552C17 16.4024 16.0154 17.5854 14.6369 17.7406C13.1147 17.9119 11.5675 17.9999 10 17.9999C8.43253 17.9999 6.88533 17.9119 5.36311 17.7406C3.98461 17.5854 3 16.4024 3 15.0552Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
