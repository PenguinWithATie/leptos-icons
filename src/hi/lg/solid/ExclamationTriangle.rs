#[cfg(feature = "HiLgSolidExclamationTriangle")]
use leptos::*;
#[cfg(feature = "HiLgSolidExclamationTriangle")]
///This icon requires the feature `HiLgSolidExclamationTriangle` to be enabled.
#[component]
pub fn ExclamationTriangle(
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
        "M9.40123 3.0034C10.5557 1.00229 13.4439 1.00229 14.5983 3.0034L21.9527 15.7509C23.1065 17.7509 21.6631 20.2501 19.3541 20.2501H4.64546C2.33649 20.2501 0.893061 17.7509 2.04691 15.7509L9.40123 3.0034ZM12 8.25C12.4142 8.25 12.75 8.58579 12.75 9V12.75C12.75 13.1642 12.4142 13.5 12 13.5C11.5858 13.5 11.25 13.1642 11.25 12.75V9C11.25 8.58579 11.5858 8.25 12 8.25ZM12 16.5C12.4142 16.5 12.75 16.1642 12.75 15.75C12.75 15.3358 12.4142 15 12 15C11.5858 15 11.25 15.3358 11.25 15.75C11.25 16.1642 11.5858 16.5 12 16.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
