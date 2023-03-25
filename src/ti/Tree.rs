#[cfg(feature = "TiTree")]
use leptos::*;
#[cfg(feature = "TiTree")]
///This icon requires the feature `TiTree` to be enabled.
#[component]
pub fn Tree(
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
        "M20.781 17.375l-2.7-3.375h.919c.373 0 .715-.207.887-.538.172-.331.146-.729-.068-1.035l-7-10c-.317-.452-.94-.562-1.393-.246-.091.063-.158.146-.221.231-.025.015-7.025 10.015-7.025 10.015-.214.306-.24.704-.068 1.035.173.331.515.538.888.538h.919l-2.7 3.375c-.24.301-.287.712-.121 1.059.167.345.518.566.902.566h7v3c0 .553.448 1 1 1s1-.447 1-1v-3h7c.384 0 .735-.221.901-.566.167-.347.12-.758-.12-1.059zm-7.781-.375v-5c0-.553-.448-1-1-1s-1 .447-1 1v5h-4.919l2.7-3.375c.24-.301.287-.712.121-1.059-.167-.345-.518-.566-.902-.566h-1.08l5.08-7.256 5.08 7.256h-1.08c-.384 0-.735.221-.901.566-.167.347-.12.758.121 1.059l2.7 3.375h-4.92z"
        /> < title > { title } < / title > < / svg >
    }
}
