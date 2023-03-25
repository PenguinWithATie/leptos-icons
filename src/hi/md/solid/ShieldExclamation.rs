#[cfg(feature = "HiMdSolidShieldExclamation")]
use leptos::*;
#[cfg(feature = "HiMdSolidShieldExclamation")]
///This icon requires the feature `HiMdSolidShieldExclamation` to be enabled.
#[component]
pub fn ShieldExclamation(
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
        "M10.3389 2.23681C10.1429 2.07406 9.85709 2.07406 9.66109 2.23681C7.72231 3.84667 5.2685 4.85772 2.58337 4.98625C2.34202 4.99781 2.13593 5.17213 2.10424 5.41168C2.03548 5.9315 2 6.46181 2 7.00041C2 12.163 5.26004 16.5641 9.83378 18.2574C9.94102 18.2971 10.0593 18.2971 10.1665 18.2574C14.7401 16.564 18 12.163 18 7.00053C18 6.46188 17.9645 5.93153 17.8957 5.41168C17.8641 5.17213 17.658 4.99781 17.4166 4.98625C14.7315 4.85772 12.2777 3.84667 10.3389 2.23681ZM10 6.00018C10.4142 6.00018 10.75 6.33597 10.75 6.75018V10.2502C10.75 10.6644 10.4142 11.0002 10 11.0002C9.58579 11.0002 9.25 10.6644 9.25 10.2502L9.25 6.75018C9.25 6.33597 9.58579 6.00018 10 6.00018ZM10 15.0002C10.5523 15.0002 11 14.5525 11 14.0002C11 13.4479 10.5523 13.0002 10 13.0002C9.44772 13.0002 9 13.4479 9 14.0002C9 14.5525 9.44772 15.0002 10 15.0002Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
