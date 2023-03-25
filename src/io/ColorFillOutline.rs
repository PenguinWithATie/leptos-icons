#[cfg(feature = "IoColorFillOutline")]
use leptos::*;
#[cfg(feature = "IoColorFillOutline")]
///This icon requires the feature `IoColorFillOutline` to be enabled.
#[component]
pub fn ColorFillOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M419.1,337.45a3.94,3.94,0,0,0-6.1,0c-10.5,12.4-45,46.55-45,77.66,0,27,21.5,48.89,48,48.89h0c26.5,0,48-22,48-48.89C464,384,429.7,349.85,419.1,337.45Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M387,287.9,155.61,58.36a36,36,0,0,0-51,0l-5.15,5.15a36,36,0,0,0,0,51l52.89,52.89,57-57L56.33,263.2a28,28,0,0,0,.3,40l131.2,126a28.05,28.05,0,0,0,38.9-.1c37.8-36.6,118.3-114.5,126.7-122.9,5.8-5.8,18.2-7.1,28.7-7.1h.3A6.53,6.53,0,0,0,387,287.9Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title
        > { title } < / title > < / svg >
    }
}
