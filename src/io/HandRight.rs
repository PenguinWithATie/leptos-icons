#[cfg(feature = "IoHandRight")]
use leptos::*;
#[cfg(feature = "IoHandRight")]
///This icon requires the feature `IoHandRight` to be enabled.
#[component]
pub fn HandRight(
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
        "M79.2,211.44h0c15.52-8.82,34.91-2.28,43.31,13.68l41.38,84.41a7,7,0,0,0,8.93,3.43h0a7,7,0,0,0,4.41-6.52V72c0-13.91,12.85-24,26.77-24s26,10.09,26,24V228.64A11.24,11.24,0,0,0,240.79,240,11,11,0,0,0,252,229V24c0-13.91,10.94-24,24.86-24S302,10.09,302,24V228.64A11.24,11.24,0,0,0,312.79,240,11,11,0,0,0,324,229V56c0-13.91,12.08-24,26-24s26,11.09,26,25V244.64A11.24,11.24,0,0,0,386.79,256,11,11,0,0,0,398,245V120c0-13.91,11.08-24,25-24s25.12,10.22,25,24V336c0,117.41-72,176-160,176H272c-88,0-115.71-39.6-136-88L67.33,255C60.67,237,63.69,220.25,79.2,211.44Z"
        /> < title > { title } < / title > < / svg >
    }
}
