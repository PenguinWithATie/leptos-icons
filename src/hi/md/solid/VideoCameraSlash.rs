#[cfg(feature = "HiMdSolidVideoCameraSlash")]
use leptos::*;
#[cfg(feature = "HiMdSolidVideoCameraSlash")]
///This icon requires the feature `HiMdSolidVideoCameraSlash` to be enabled.
#[component]
pub fn VideoCameraSlash(
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
        "M1 13.75V7.18198L9.81802 16H3.25C2.00736 16 1 14.9926 1 13.75Z" fill = "#0F172A"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 6.25V12.818L4.18198 4H10.75C11.9926 4 13 5.00736 13 6.25Z" fill = "#0F172A"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 4.75002C19 4.44668 18.8173 4.1732 18.537 4.05711C18.2568 3.94103 17.9342 4.00519 17.7197 4.21969L14.7197 7.21969C14.579 7.36034 14.5 7.55111 14.5 7.75002V12.25C14.5 12.4489 14.579 12.6397 14.7197 12.7804L17.7197 15.7804C17.9342 15.9949 18.2568 16.059 18.537 15.9429C18.8173 15.8268 19 15.5534 19 15.25V4.75002Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.28033 4.21967C1.98744 3.92678 1.51256 3.92678 1.21967 4.21967C0.926777 4.51256 0.926777 4.98744 1.21967 5.28033L11.7197 15.7803C12.0126 16.0732 12.4874 16.0732 12.7803 15.7803C13.0732 15.4874 13.0732 15.0126 12.7803 14.7197L2.28033 4.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
