#[cfg(feature = "SiKlook")]
use leptos::*;
#[cfg(feature = "SiKlook")]
///This icon requires the feature `SiKlook` to be enabled.
#[component]
pub fn Klook(
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
        "M4.8 0A4.79 4.79 0 0 0 0 4.8v14.4C0 21.86 2.14 24 4.8 24h14.4c2.66 0 4.8-2.14 4.8-4.8V4.8C24 2.14 21.86 0 19.2 0H4.8zM12 3.449v.001c4.242 0 7.833 1.904 7.833 6.17 0 2.932-3.86 7.815-6.164 10.164-.99 1.008-2.32 1.036-3.338 0-2.303-2.349-6.164-7.232-6.164-10.164 0-4.162 3.476-6.171 7.833-6.171zm3.54 2.155l-5.05 4.96 5.05 4.956a1.84 1.84 0 0 0 0-2.634v-.001l-2.366-2.323 2.366-2.323a1.84 1.84 0 0 0 0-2.635zm-7.349.144v9.772a1.86 1.86 0 0 0 1.868-1.852V7.602a1.86 1.86 0 0 0-1.866-1.854h-.002z"
        /> < title > { title } < / title > < / svg >
    }
}
