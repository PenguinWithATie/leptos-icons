#[cfg(feature = "SiIntegromat")]
use leptos::*;
#[cfg(feature = "SiIntegromat")]
///This icon requires the feature `SiIntegromat` to be enabled.
#[component]
pub fn Integromat(
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
        "M12 0c-.681 0-1.349.057-2 .166v4.09a8.002 8.002 0 012-.253c.69 0 1.36.088 2 .253V.166C13.35.056 12.68 0 12 0zM8.002.683C3.342 2.332 0 6.78 0 12c0 6.623 5.377 12 12 12s12-5.377 12-12C24 6.78 20.658 2.332 15.999.683v4.392a7.997 7.997 0 11-7.997 0zM12 6.003c-.7 0-1.374.12-2 .342v9.32a5.98 5.98 0 002 .343c.7 0 1.374-.121 2-.342V6.345a5.977 5.977 0 00-2-.342z"
        /> < title > { title } < / title > < / svg >
    }
}
