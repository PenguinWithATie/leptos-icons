#[cfg(feature = "IoSnow")]
use leptos::*;
#[cfg(feature = "IoSnow")]
///This icon requires the feature `IoSnow` to be enabled.
#[component]
pub fn Snow(
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
        "M461,349l-34-19.64a89.53,89.53,0,0,1,20.94-16,22,22,0,0,0-21.28-38.51,133.62,133.62,0,0,0-38.55,32.1L300,256l88.09-50.86a133.46,133.46,0,0,0,38.55,32.1,22,22,0,1,0,21.28-38.51,89.74,89.74,0,0,1-20.94-16l34-19.64A22,22,0,1,0,439,125l-34,19.63a89.74,89.74,0,0,1-3.42-26.15A22,22,0,0,0,380,96h-.41a22,22,0,0,0-22,21.59A133.61,133.61,0,0,0,366.09,167L278,217.89V116.18a133.5,133.5,0,0,0,47.07-17.33,22,22,0,0,0-22.71-37.69A89.56,89.56,0,0,1,278,71.27V38a22,22,0,0,0-44,0V71.27a89.56,89.56,0,0,1-24.36-10.11,22,22,0,1,0-22.71,37.69A133.5,133.5,0,0,0,234,116.18V217.89L145.91,167a133.61,133.61,0,0,0,8.52-49.43,22,22,0,0,0-22-21.59H132a22,22,0,0,0-21.59,22.41A89.74,89.74,0,0,1,107,144.58L73,125a22,22,0,1,0-22,38.1l34,19.64a89.74,89.74,0,0,1-20.94,16,22,22,0,1,0,21.28,38.51,133.62,133.62,0,0,0,38.55-32.1L212,256l-88.09,50.86a133.62,133.62,0,0,0-38.55-32.1,22,22,0,1,0-21.28,38.51,89.74,89.74,0,0,1,20.94,16L51,349a22,22,0,1,0,22,38.1l34-19.63a89.74,89.74,0,0,1,3.42,26.15A22,22,0,0,0,132,416h.41a22,22,0,0,0,22-21.59A133.61,133.61,0,0,0,145.91,345L234,294.11V395.82a133.5,133.5,0,0,0-47.07,17.33,22,22,0,1,0,22.71,37.69A89.56,89.56,0,0,1,234,440.73V474a22,22,0,0,0,44,0V440.73a89.56,89.56,0,0,1,24.36,10.11,22,22,0,0,0,22.71-37.69A133.5,133.5,0,0,0,278,395.82V294.11L366.09,345a133.61,133.61,0,0,0-8.52,49.43,22,22,0,0,0,22,21.59H380a22,22,0,0,0,21.59-22.41A89.74,89.74,0,0,1,405,367.42l34,19.63A22,22,0,1,0,461,349Z"
        /> < title > { title } < / title > < / svg >
    }
}
