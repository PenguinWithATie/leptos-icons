#[cfg(feature = "FaBrandsCottonBureau")]
use leptos::*;
#[cfg(feature = "FaBrandsCottonBureau")]
///This icon requires the feature `FaBrandsCottonBureau` to be enabled.
#[component]
pub fn CottonBureau(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M474.31 330.41c-23.66 91.85-94.23 144.59-201.9 148.35V429.6c0-48 26.41-74.39 74.39-74.39 62 0 99.2-37.2 99.2-99.21 0-61.37-36.53-98.28-97.38-99.06-33-69.32-146.5-64.65-177.24 0C110.52 157.72 74 194.63 74 256c0 62.13 37.27 99.41 99.4 99.41 48 0 74.55 26.23 74.55 74.39V479c-134.43-5-211.1-85.07-211.1-223 0-141.82 81.35-223.2 223.2-223.2 114.77 0 189.84 53.2 214.69 148.81H500C473.88 71.51 388.22 8 259.82 8 105 8 12 101.19 12 255.82 12 411.14 105.19 504.34 259.82 504c128.27 0 213.87-63.81 239.67-173.59zM357 182.33c41.37 3.45 64.2 29 64.2 73.67 0 48-26.43 74.41-74.4 74.41-28.61 0-49.33-9.59-61.59-27.33 83.06-16.55 75.59-99.67 71.79-120.75zm-81.68 97.36c-2.46-10.34-16.33-87 56.23-97 2.27 10.09 16.52 87.11-56.26 97zM260 132c28.61 0 49 9.67 61.44 27.61-28.36 5.48-49.36 20.59-61.59 43.45-12.23-22.86-33.23-38-61.6-43.45 12.41-17.69 33.27-27.35 61.57-27.35zm-71.52 50.72c73.17 10.57 58.91 86.81 56.49 97-72.41-9.84-59-86.95-56.25-97zM173.2 330.41c-48 0-74.4-26.4-74.4-74.41 0-44.36 22.86-70 64.22-73.67-6.75 37.2-1.38 106.53 71.65 120.75-12.14 17.63-32.84 27.3-61.14 27.3zm53.21 12.39A80.8 80.8 0 0 0 260 309.25c7.77 14.49 19.33 25.54 33.82 33.55a80.28 80.28 0 0 0-33.58 33.83c-8-14.5-19.07-26.23-33.56-33.83z"
        /> < title > { title } < / title > < / svg >
    }
}
