#[cfg(feature = "IoFlashlight")]
use leptos::*;
#[cfg(feature = "IoFlashlight")]
///This icon requires the feature `IoFlashlight` to be enabled.
#[component]
pub fn Flashlight(
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
        "M462,216c9.35-9.35,15.14-19.09,17.19-28.95,2.7-12.95-1.29-25.55-11.22-35.48L360.43,44.05C346.29,29.92,322,24.07,296,50l-2,2a8,8,0,0,0,0,11.32L448.64,218A8,8,0,0,0,460,218Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M250.14,153.08l-.16,2.34c-.53,7.18-6.88,19.15-13.88,26.14L47.27,370.36c-11.12,11.11-16.46,25.57-15.05,40.7C33.49,424.58,40.16,438,51,448.83L63.17,461c12.61,12.6,27.78,19,42.49,19a50.4,50.4,0,0,0,36-15.24l188.84-188.8c7.07-7.07,18.84-13.3,26.17-13.87,17.48-1.32,43.57-3.28,67.79-15.65a4,4,0,0,0,1-6.37L271.69,86.31a4,4,0,0,0-6.39,1C253.18,110.3,251.48,134.22,250.14,153.08Zm-9.95,146.83a20,20,0,1,1,0-25.25A20,20,0,0,1,240.19,299.91Z"
        /> < title > { title } < / title > < / svg >
    }
}
