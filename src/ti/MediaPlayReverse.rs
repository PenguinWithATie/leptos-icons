#[cfg(feature = "TiMediaPlayReverse")]
use leptos::*;
#[cfg(feature = "TiMediaPlayReverse")]
///This icon requires the feature `TiMediaPlayReverse` to be enabled.
#[component]
pub fn MediaPlayReverse(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 19c1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2-.5 0-1 .2-1.4.6-2.6 2.5-6.6 6.4-6.6 6.4s4 3.9 6.6 6.4c.4.4.9.6 1.4.6z"
        /> < title > { title } < / title > < / svg >
    }
}
