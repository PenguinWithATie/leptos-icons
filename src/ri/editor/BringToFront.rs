#[cfg(feature = "RiEditorBringToFront")]
use leptos::*;
#[cfg(feature = "RiEditorBringToFront")]
///This icon requires the feature `RiEditorBringToFront` to be enabled.
#[component]
pub fn BringToFront(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M11 3c.552 0 1 .448 1 1v2h5c.552 0 1 .448 1 1v5h2c.552 0 1 .448 1 1v7c0 .552-.448 1-1 1h-7c-.552 0-1-.448-1-1v-2H7c-.552 0-1-.448-1-1v-5H4c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h7zm5 5H8v8h8V8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
