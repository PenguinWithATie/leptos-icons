#[cfg(feature = "IoDocumentAttachSharp")]
use leptos::*;
#[cfg(feature = "IoDocumentAttachSharp")]
///This icon requires the feature `IoDocumentAttachSharp` to be enabled.
#[component]
pub fn DocumentAttachSharp(
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
        "M280,240a8,8,0,0,1-8-8V48H214.75a65.42,65.42,0,0,0-6.5-9.81C196.72,23.88,179.59,16,160,16c-37.68,0-64,29.61-64,72V232c0,25,20.34,40,40,40a39.57,39.57,0,0,0,40-40V80H144V232a7.75,7.75,0,0,1-8,8c-2.23,0-8-1.44-8-8V88c0-19.34,8.41-40,32-40,29.69,0,32,30.15,32,39.38V226.13c0,17.45-5.47,33.23-15.41,44.46C166.5,282,152.47,288,136,288s-30.5-6-40.59-17.41C85.47,259.36,80,243.58,80,226.13V144H48v82.13c0,51.51,33.19,89.63,80,93.53V468a12,12,0,0,0,12,12H452a12,12,0,0,0,12-12V240Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M308,208H454.31a2,2,0,0,0,1.42-3.41L307.41,56.27A2,2,0,0,0,304,57.69V204A4,4,0,0,0,308,208Z"
        /> < title > { title } < / title > < / svg >
    }
}
