#[cfg(feature = "HiMdSolidAdjustmentsHorizontal")]
use leptos::*;
#[cfg(feature = "HiMdSolidAdjustmentsHorizontal")]
///This icon requires the feature `HiMdSolidAdjustmentsHorizontal` to be enabled.
#[component]
pub fn AdjustmentsHorizontal(
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
        "M10 3.75C10 2.64543 9.10457 1.75 8 1.75C6.89543 1.75 6 2.64543 6 3.75C6 4.85457 6.89543 5.75 8 5.75C9.10457 5.75 10 4.85457 10 3.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.25 4.5C17.6642 4.5 18 4.16421 18 3.75C18 3.33579 17.6642 3 17.25 3L11.75 3C11.3358 3 11 3.33579 11 3.75C11 4.16421 11.3358 4.5 11.75 4.5L17.25 4.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 3.75C5 4.16421 4.66421 4.5 4.25 4.5H2.75C2.33579 4.5 2 4.16421 2 3.75C2 3.33579 2.33579 3 2.75 3L4.25 3C4.66421 3 5 3.33579 5 3.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.25 17C4.66421 17 5 16.6642 5 16.25C5 15.8358 4.66421 15.5 4.25 15.5H2.75C2.33579 15.5 2 15.8358 2 16.25C2 16.6642 2.33579 17 2.75 17H4.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.25 17C17.6642 17 18 16.6642 18 16.25C18 15.8358 17.6642 15.5 17.25 15.5H11.75C11.3358 15.5 11 15.8358 11 16.25C11 16.6642 11.3358 17 11.75 17H17.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 10C9 10.4142 8.66421 10.75 8.25 10.75H2.75C2.33579 10.75 2 10.4142 2 10C2 9.58579 2.33579 9.25 2.75 9.25L8.25 9.25C8.66421 9.25 9 9.58579 9 10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.25 10.75C17.6642 10.75 18 10.4142 18 10C18 9.58579 17.6642 9.25 17.25 9.25H15.75C15.3358 9.25 15 9.58579 15 10C15 10.4142 15.3358 10.75 15.75 10.75H17.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 10C14 8.89543 13.1046 8 12 8C10.8954 8 10 8.89543 10 10C10 11.1046 10.8954 12 12 12C13.1046 12 14 11.1046 14 10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 16.25C10 15.1454 9.10457 14.25 8 14.25C6.89543 14.25 6 15.1454 6 16.25C6 17.3546 6.89543 18.25 8 18.25C9.10457 18.25 10 17.3546 10 16.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
