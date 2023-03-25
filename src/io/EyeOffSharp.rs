#[cfg(feature = "IoEyeOffSharp")]
use leptos::*;
#[cfg(feature = "IoEyeOffSharp")]
///This icon requires the feature `IoEyeOffSharp` to be enabled.
#[component]
pub fn EyeOffSharp(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "240.44" y = "0.03" width = "31.11" height = "511.95" transform =
        "translate(-106.04 256) rotate(-45)" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M259.34,192.09l60.57,60.57A64.07,64.07,0,0,0,259.34,192.09Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M252.66,319.91l-60.57-60.57A64.07,64.07,0,0,0,252.66,319.91Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,352a96,96,0,0,1-92.6-121.34L94.33,161.58C66.12,187.42,39.24,221.14,16,256c26.42,44,62.56,89.24,100.2,115.18C159.38,400.92,206.33,416,255.76,416A233.47,233.47,0,0,0,335,402.2l-53.61-53.6A95.84,95.84,0,0,1,256,352Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,160a96,96,0,0,1,92.6,121.34L419.26,352c29.15-26.25,56.07-61.56,76.74-96-26.38-43.43-62.9-88.56-101.18-114.82C351.1,111.2,304.31,96,255.76,96a222.92,222.92,0,0,0-78.21,14.29l53.11,53.11A95.84,95.84,0,0,1,256,160Z"
        /> < title > { title } < / title > < / svg >
    }
}
