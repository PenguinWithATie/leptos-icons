#[cfg(feature = "HiLgSolidPresentationChartBar")]
use leptos::*;
#[cfg(feature = "HiLgSolidPresentationChartBar")]
///This icon requires the feature `HiLgSolidPresentationChartBar` to be enabled.
#[component]
pub fn PresentationChartBar(
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
        "M2.25 2.25C1.83579 2.25 1.5 2.58579 1.5 3C1.5 3.41421 1.83579 3.75 2.25 3.75H3V14.25C3 15.9069 4.34315 17.25 6 17.25H7.20943L6.03849 20.7628C5.9075 21.1558 6.11987 21.5805 6.51283 21.7115C6.90579 21.8425 7.33053 21.6301 7.46151 21.2372L7.79057 20.25H16.2094L16.5385 21.2372C16.6695 21.6301 17.0942 21.8425 17.4872 21.7115C17.8801 21.5805 18.0925 21.1558 17.9615 20.7628L16.7906 17.25H18C19.6569 17.25 21 15.9069 21 14.25V3.75H21.75C22.1642 3.75 22.5 3.41421 22.5 3C22.5 2.58579 22.1642 2.25 21.75 2.25H2.25ZM8.29057 18.75L8.79057 17.25H15.2094L15.7094 18.75H8.29057ZM15.75 6.75C15.75 6.33579 15.4142 6 15 6C14.5858 6 14.25 6.33579 14.25 6.75V12.75C14.25 13.1642 14.5858 13.5 15 13.5C15.4142 13.5 15.75 13.1642 15.75 12.75V6.75ZM12.75 9C12.75 8.58579 12.4142 8.25 12 8.25C11.5858 8.25 11.25 8.58579 11.25 9V12.75C11.25 13.1642 11.5858 13.5 12 13.5C12.4142 13.5 12.75 13.1642 12.75 12.75V9ZM9.75 11.25C9.75 10.8358 9.41421 10.5 9 10.5C8.58579 10.5 8.25 10.8358 8.25 11.25V12.75C8.25 13.1642 8.58579 13.5 9 13.5C9.41421 13.5 9.75 13.1642 9.75 12.75V11.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
