#[cfg(feature = "BiLogos99designs")]
use leptos::*;
#[cfg(feature = "BiLogos99designs")]
///This icon requires the feature `BiLogos99designs` to be enabled.
#[component]
pub fn _99designs(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19.93 10.61A2.7 2.7 0 0 0 19 10a2.74 2.74 0 0 0-1.1-.19 3.28 3.28 0 0 0-2.16.76v-.05a3.67 3.67 0 0 0-5.09-3.39 3.61 3.61 0 0 0-1.78 1.56 3.67 3.67 0 0 0-3.12-1.86 3.74 3.74 0 0 0-1.82.44 3.66 3.66 0 0 0-1.37 1.28A3.77 3.77 0 0 0 2 10.34a3.67 3.67 0 0 0 3.42 3.83l-1.6 2.76h2.39l2.65-4.59a3.63 3.63 0 0 0 2.93 1.84l-1.59 2.76h2.4l1.86-3.23a3.53 3.53 0 0 0 2.07 3.19 3.52 3.52 0 0 0 2.61.05 2.58 2.58 0 0 0 .9-.74v.73h2V7.06h-2.11zm-12.78.72a1.69 1.69 0 0 1-1.46.83 1.72 1.72 0 0 1-.86-.16 1.75 1.75 0 0 1-.62-.62 1.78 1.78 0 0 1-.21-.87 1.69 1.69 0 0 1 1.67-1.69 1.63 1.63 0 0 1 .84.23 1.58 1.58 0 0 1 .62.61 1.62 1.62 0 0 1 .23.83 1.72 1.72 0 0 1-.21.84zm6.38 0a1.58 1.58 0 0 1-.62.61 1.53 1.53 0 0 1-.84.22 1.78 1.78 0 0 1-.84-.22 1.65 1.65 0 0 1-.61-.62 1.67 1.67 0 0 1 0-1.69 1.73 1.73 0 0 1 .62-.61 1.63 1.63 0 0 1 .84-.23 1.69 1.69 0 0 1 .84.23 1.69 1.69 0 0 1 .63 2.28zm6.3 3a1.72 1.72 0 0 1-2 .85 1.72 1.72 0 0 1-1-.7 1.75 1.75 0 0 1-.29-1.15 1.77 1.77 0 0 1 .51-1.07 1.75 1.75 0 0 1 2.35-.11 1.73 1.73 0 0 1 .43 2.18z"
        /> < title > { title } < / title > < / svg >
    }
}
