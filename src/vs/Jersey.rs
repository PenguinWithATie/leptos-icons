#[cfg(feature = "VsJersey")]
use leptos::*;
#[cfg(feature = "VsJersey")]
///This icon requires the feature `VsJersey` to be enabled.
#[component]
pub fn Jersey(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.91 14.22H4.06l-.5-.5V7.06H2.15l-.48-.38L1 4l.33-.6L5.59 2l.64.32a2.7 2.7 0 0 0 .21.44c.071.103.152.2.24.29.168.169.369.302.59.39a1.82 1.82 0 0 0 1.43 0 1.74 1.74 0 0 0 .59-.39c.09-.095.173-.195.25-.3l.15-.29a1.21 1.21 0 0 0 .05-.14l.64-.32 4.26 1.42L15 4l-.66 2.66-.49.38h-1.44v6.66l-.5.52zm-7.35-1h6.85V6.56l.5-.5h1.52l.46-1.83-3.4-1.14a1.132 1.132 0 0 1-.12.21c-.11.161-.233.312-.37.45a2.75 2.75 0 0 1-.91.61 2.85 2.85 0 0 1-2.22 0A2.92 2.92 0 0 1 6 3.75a2.17 2.17 0 0 1-.36-.44l-.13-.22-3.43 1.14.46 1.83h1.52l.5.5v6.66z"
        /> < title > { title } < / title > < / svg >
    }
}
