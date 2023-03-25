#[cfg(feature = "SiHotjar")]
use leptos::*;
#[cfg(feature = "SiHotjar")]
///This icon requires the feature `SiHotjar` to be enabled.
#[component]
pub fn Hotjar(
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
        "M21.055 7.814C17.512 1.404 7.118 0 7.118 0s4.798 5.34-1.334 9.567c-3.876 2.666-5.41 6.13-3.75 9.914 1.27 2.9 3.96 4.076 6.86 4.519-.745-1.434-.932-3.505-.381-5.628.055-.212.116-.434.186-.636.813 1.258 2.148 1.946 3.45 1.629 1.783-.424 2.829-2.582 2.342-4.799a5.104 5.104 0 00-.536-1.372c.07.017.14.024.212.047 2.225.635 3.301 3.962 2.403 7.434a9.266 9.266 0 01-1.325 2.946c3.82-1.23 6.36-4.311 7.06-7.056.736-2.856.177-6.185-1.25-8.751z"
        /> < title > { title } < / title > < / svg >
    }
}
