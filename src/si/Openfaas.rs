#[cfg(feature = "SiOpenfaas")]
use leptos::*;
#[cfg(feature = "SiOpenfaas")]
///This icon requires the feature `SiOpenfaas` to be enabled.
#[component]
pub fn Openfaas(
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
        "M4.304 0A4.31 4.31 0 0 0 0 4.304v15.391A4.31 4.31 0 0 0 4.304 24h15.392A4.31 4.31 0 0 0 24 19.695V4.305A4.31 4.31 0 0 0 19.695 0zm-.006.776h15.398a3.532 3.532 0 0 1 3.528 3.528v15.391c0 .205-.019.409-.054.61-1.234-.015-1.858-.412-2.514-.834-.708-.454-1.51-.97-3.04-.97s-2.33.516-3.038.97c-.17.11-.338.217-.514.317a43.042 43.042 0 0 1-.775-1.907.806.806 0 0 1 .01-.739c.3-.489 1.23-.691 1.572-.729.361-.027 5.97-.555 6.306-7.153a.42.42 0 0 0-.72-.317c-.457.464-1.886 1.634-3 1.47a11.06 11.06 0 0 1-.693-.127c-1.425-.293-3.339-.685-4.972 1.72-1.633-2.406-3.548-2.012-4.972-1.72-.248.05-.48.098-.697.128-1.108.162-2.538-1.007-2.997-1.471a.42.42 0 0 0-.72.316c.336 6.58 5.914 7.124 6.304 7.153.086.012 1.1.16 1.48.717.15.237.184.529.092.793a19.35 19.35 0 0 1-.552 1.747c-.107-.065-.214-.13-.32-.198-.708-.454-1.51-.97-3.039-.97-1.53 0-2.332.516-3.04.97-.654.42-1.277.816-2.5.834a3.532 3.532 0 0 1-.055-.61V4.305A3.532 3.532 0 0 1 4.298.775zm4.474 2.108l-4.74 1.429 1.293 4.288 3.165-.955-.19 1.229 4.894.757.684-4.426-4.62-.715zm10.494.813l-4.938.348.315 4.466 4.938-.347Z"
        /> < title > { title } < / title > < / svg >
    }
}
