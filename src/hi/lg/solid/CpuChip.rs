#[cfg(feature = "HiLgSolidCpuChip")]
use leptos::*;
#[cfg(feature = "HiLgSolidCpuChip")]
///This icon requires the feature `HiLgSolidCpuChip` to be enabled.
#[component]
pub fn CpuChip(
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
        "M16.5 7.5H7.5V16.5H16.5V7.5Z" fill = "#0F172A" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M8.25 2.25C8.66421 2.25 9 2.58579 9 3V3.75H11.25V3C11.25 2.58579 11.5858 2.25 12 2.25C12.4142 2.25 12.75 2.58579 12.75 3V3.75H15V3C15 2.58579 15.3358 2.25 15.75 2.25C16.1642 2.25 16.5 2.58579 16.5 3V3.75H17.25C18.9069 3.75 20.25 5.09315 20.25 6.75V7.5H21C21.4142 7.5 21.75 7.83579 21.75 8.25C21.75 8.66421 21.4142 9 21 9H20.25V11.25H21C21.4142 11.25 21.75 11.5858 21.75 12C21.75 12.4142 21.4142 12.75 21 12.75H20.25V15H21C21.4142 15 21.75 15.3358 21.75 15.75C21.75 16.1642 21.4142 16.5 21 16.5H20.25V17.25C20.25 18.9069 18.9069 20.25 17.25 20.25H16.5V21C16.5 21.4142 16.1642 21.75 15.75 21.75C15.3358 21.75 15 21.4142 15 21V20.25H12.75V21C12.75 21.4142 12.4142 21.75 12 21.75C11.5858 21.75 11.25 21.4142 11.25 21V20.25H9V21C9 21.4142 8.66421 21.75 8.25 21.75C7.83579 21.75 7.5 21.4142 7.5 21V20.25H6.75C5.09315 20.25 3.75 18.9069 3.75 17.25V16.5H3C2.58579 16.5 2.25 16.1642 2.25 15.75C2.25 15.3358 2.58579 15 3 15H3.75V12.75H3C2.58579 12.75 2.25 12.4142 2.25 12C2.25 11.5858 2.58579 11.25 3 11.25H3.75V9H3C2.58579 9 2.25 8.66421 2.25 8.25C2.25 7.83579 2.58579 7.5 3 7.5H3.75V6.75C3.75 5.09315 5.09315 3.75 6.75 3.75H7.5V3C7.5 2.58579 7.83579 2.25 8.25 2.25ZM6 6.75C6 6.33579 6.33579 6 6.75 6H17.25C17.6642 6 18 6.33579 18 6.75V17.25C18 17.6642 17.6642 18 17.25 18H6.75C6.33579 18 6 17.6642 6 17.25V6.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
