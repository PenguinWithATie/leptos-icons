#[cfg(feature = "SiDataverse")]
use leptos::*;
#[cfg(feature = "SiDataverse")]
///This icon requires the feature `SiDataverse` to be enabled.
#[component]
pub fn Dataverse(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.327 4.568a3.895 3.895 0 0 1 6.671.82h.001c1.996 4.686.925 10.086-2.487 13.154-3.772 3.39-8.875 3.408-12.178.796a.54.54 0 0 0 .05-.072l1.697-2.939c1.596.819 3.11 1.04 4.439.739 1.472-.333 2.734-1.304 3.64-2.872 1.708-2.96 1.161-6.961-1.777-9.583a.396.396 0 0 0-.056-.043ZM7.673 19.432a3.895 3.895 0 0 1-6.661-.798H1.01C-.999 13.944.071 8.53 3.488 5.458c3.772-3.391 8.876-3.408 12.18-.795a.51.51 0 0 0-.051.071l-1.697 2.94c-1.595-.82-3.109-1.04-4.439-.739-1.472.333-2.734 1.304-3.639 2.872-1.709 2.959-1.162 6.961 1.776 9.582.018.016.036.03.055.043Zm1.969-3.345a4.72 4.72 0 0 1 1.612-8.746c-2.064-.234-3.829.723-4.979 2.716-1.598 2.767-1.072 6.507 1.676 8.959l1.691-2.929Zm4.718-8.174h.001a4.72 4.72 0 0 1 1.727 6.447 4.712 4.712 0 0 1-3.34 2.3c2.064.233 3.828-.724 4.979-2.716 1.598-2.768 1.071-6.508-1.676-8.96L14.36 7.913Zm-4.468 7.741a4.219 4.219 0 0 1 4.217-7.308h.002a4.22 4.22 0 0 1-4.188 7.326l-.031-.018Z"
        /> < title > { title } < / title > < / svg >
    }
}
