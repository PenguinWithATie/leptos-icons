#[cfg(feature = "HiLgSolidUserPlus")]
use leptos::*;
#[cfg(feature = "HiLgSolidUserPlus")]
///This icon requires the feature `HiLgSolidUserPlus` to be enabled.
#[component]
pub fn UserPlus(
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
        "M6.25 6.375C6.25 4.09683 8.09683 2.25 10.375 2.25C12.6532 2.25 14.5 4.09683 14.5 6.375C14.5 8.65317 12.6532 10.5 10.375 10.5C8.09683 10.5 6.25 8.65317 6.25 6.375Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.25 19.125C3.25 15.19 6.43997 12 10.375 12C14.31 12 17.5 15.19 17.5 19.125V19.1276C17.5 19.1674 17.4996 19.2074 17.499 19.2469C17.4946 19.5054 17.3574 19.7435 17.1359 19.8768C15.1607 21.0661 12.8466 21.75 10.375 21.75C7.90343 21.75 5.5893 21.0661 3.61406 19.8768C3.39256 19.7435 3.25537 19.5054 3.25103 19.2469C3.25034 19.2064 3.25 19.1657 3.25 19.125Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.75 7.5C19.75 7.08579 19.4142 6.75 19 6.75C18.5858 6.75 18.25 7.08579 18.25 7.5V9.75H16C15.5858 9.75 15.25 10.0858 15.25 10.5C15.25 10.9142 15.5858 11.25 16 11.25H18.25V13.5C18.25 13.9142 18.5858 14.25 19 14.25C19.4142 14.25 19.75 13.9142 19.75 13.5V11.25H22C22.4142 11.25 22.75 10.9142 22.75 10.5C22.75 10.0858 22.4142 9.75 22 9.75H19.75V7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
