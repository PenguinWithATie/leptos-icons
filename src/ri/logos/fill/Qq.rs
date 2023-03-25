#[cfg(feature = "RiLogosFillQq")]
use leptos::*;
#[cfg(feature = "RiLogosFillQq")]
///This icon requires the feature `RiLogosFillQq` to be enabled.
#[component]
pub fn Qq(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M19.913 14.529a31.977 31.977 0 0 0-.675-1.886l-.91-2.246c0-.026.012-.468.012-.696C18.34 5.86 16.507 2 12 2 7.493 2 5.66 5.86 5.66 9.7c0 .229.011.671.012.697l-.91 2.246c-.248.643-.495 1.312-.675 1.886-.86 2.737-.581 3.87-.369 3.895.455.054 1.771-2.06 1.771-2.06 0 1.224.637 2.822 2.016 3.976-.515.157-1.147.399-1.554.695-.365.267-.319.54-.253.65.289.481 4.955.307 6.303.157 1.347.15 6.014.324 6.302-.158.066-.11.112-.382-.253-.649-.407-.296-1.039-.538-1.555-.696 1.379-1.153 2.016-2.751 2.016-3.976 0 0 1.316 2.115 1.771 2.06.212-.025.49-1.157-.37-3.894"
        /></ g > < title > { title } < / title > < / svg >
    }
}
