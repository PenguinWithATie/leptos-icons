#[cfg(feature = "HiMdSolidAdjustmentsVertical")]
use leptos::*;
#[cfg(feature = "HiMdSolidAdjustmentsVertical")]
///This icon requires the feature `HiMdSolidAdjustmentsVertical` to be enabled.
#[component]
pub fn AdjustmentsVertical(
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
        "M17 2.75C17 2.33579 16.6642 2 16.25 2C15.8358 2 15.5 2.33579 15.5 2.75V8.25C15.5 8.66421 15.8358 9 16.25 9C16.6642 9 17 8.66421 17 8.25V2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 15.75C17 15.3358 16.6642 15 16.25 15C15.8358 15 15.5 15.3358 15.5 15.75V17.25C15.5 17.6642 15.8358 18 16.25 18C16.6642 18 17 17.6642 17 17.25V15.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 15C4.16421 15 4.5 15.3358 4.5 15.75V17.25C4.5 17.6642 4.16421 18 3.75 18C3.33579 18 3 17.6642 3 17.25V15.75C3 15.3358 3.33579 15 3.75 15Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.5 2.75C4.5 2.33579 4.16421 2 3.75 2C3.33579 2 3 2.33579 3 2.75V8.25C3 8.66421 3.33579 9 3.75 9C4.16421 9 4.5 8.66421 4.5 8.25V2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 11C10.4142 11 10.75 11.3358 10.75 11.75V17.25C10.75 17.6642 10.4142 18 10 18C9.58579 18 9.25 17.6642 9.25 17.25V11.75C9.25 11.3358 9.58579 11 10 11Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.75 2.75C10.75 2.33579 10.4142 2 10 2C9.58579 2 9.25 2.33579 9.25 2.75V4.25C9.25 4.66421 9.58579 5 10 5C10.4142 5 10.75 4.66421 10.75 4.25V2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 6C8.89543 6 8 6.89543 8 8C8 9.10457 8.89543 10 10 10C11.1046 10 12 9.10457 12 8C12 6.89543 11.1046 6 10 6Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 10C2.64543 10 1.75 10.8954 1.75 12C1.75 13.1046 2.64543 14 3.75 14C4.85457 14 5.75 13.1046 5.75 12C5.75 10.8954 4.85457 10 3.75 10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.25 10C15.1454 10 14.25 10.8954 14.25 12C14.25 13.1046 15.1454 14 16.25 14C17.3546 14 18.25 13.1046 18.25 12C18.25 10.8954 17.3546 10 16.25 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
