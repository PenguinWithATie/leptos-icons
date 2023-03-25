#[cfg(feature = "HiMdSolidBattery100")]
use leptos::*;
#[cfg(feature = "HiMdSolidBattery100")]
///This icon requires the feature `HiMdSolidBattery100` to be enabled.
#[component]
pub fn Battery100(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.75 8C4.33579 8 4 8.33579 4 8.75V11.25C4 11.6642 4.33579 12 4.75 12H14.25C14.6642 12 15 11.6642 15 11.25V8.75C15 8.33579 14.6642 8 14.25 8H4.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 7.25C1 6.00736 2.00736 5 3.25 5H15.75C16.9926 5 18 6.00736 18 7.25V8.33535C18.5826 8.54127 19 9.09689 19 9.75V10.25C19 10.9031 18.5826 11.4587 18 11.6646V12.75C18 13.9926 16.9926 15 15.75 15H3.25C2.00736 15 1 13.9926 1 12.75V7.25ZM3.25 6.5C2.83579 6.5 2.5 6.83579 2.5 7.25V12.75C2.5 13.1642 2.83579 13.5 3.25 13.5H15.75C16.1642 13.5 16.5 13.1642 16.5 12.75V7.25C16.5 6.83579 16.1642 6.5 15.75 6.5H3.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
