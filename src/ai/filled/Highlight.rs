#[cfg(feature = "AiFilledHighlight")]
use leptos::*;
#[cfg(feature = "AiFilledHighlight")]
///This icon requires the feature `AiFilledHighlight` to be enabled.
#[component]
pub fn Highlight(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M957.6 507.4L603.2 158.2a7.9 7.9 0 0 0-11.2 0L353.3 393.4a8.03 8.03 0 0 0-.1 11.3l.1.1 40 39.4-117.2 115.3a8.03 8.03 0 0 0-.1 11.3l.1.1 39.5 38.9-189.1 187H72.1c-4.4 0-8.1 3.6-8.1 8V860c0 4.4 3.6 8 8 8h344.9c2.1 0 4.1-.8 5.6-2.3l76.1-75.6 40.4 39.8a7.9 7.9 0 0 0 11.2 0l117.1-115.6 40.1 39.5a7.9 7.9 0 0 0 11.2 0l238.7-235.2c3.4-3 3.4-8 .3-11.2z"
        /> < title > { title } < / title > < / svg >
    }
}
