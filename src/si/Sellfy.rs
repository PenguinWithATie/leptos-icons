#[cfg(feature = "SiSellfy")]
use leptos::*;
#[cfg(feature = "SiSellfy")]
///This icon requires the feature `SiSellfy` to be enabled.
#[component]
pub fn Sellfy(
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
        "M23.179.818C15.533-.273 8.406-.273.8.818-.266 8.377-.266 15.424.8 22.946 4.511 23.491 8.22 24 12.005 24c3.748 0 7.459-.51 11.17-1.017 1.1-7.56 1.1-14.607 0-22.165h.004zm-11.54 18.314c-2.055 0-4.226-.689-5.179-1.199l.807-3.126c1.064.705 2.682 1.395 4.446 1.395 1.395 0 2.24-.436 2.24-1.305 0-.615-.435-.975-1.575-1.26l-2.279-.631c-2.416-.66-3.557-1.891-3.557-3.855 0-2.365 1.83-4.256 5.619-4.256 1.99 0 3.973.545 5.07 1.092l-.951 2.976c-1.104-.615-2.79-1.125-4.226-1.125-1.365 0-1.95.436-1.95 1.092 0 .619.404.87 1.291 1.092l2.488.734c2.566.736 3.707 1.966 3.707 3.885-.076 2.701-2.461 4.517-5.957 4.517l.006-.026z"
        /> < title > { title } < / title > < / svg >
    }
}
