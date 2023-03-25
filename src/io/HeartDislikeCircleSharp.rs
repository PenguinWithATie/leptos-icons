#[cfg(feature = "IoHeartDislikeCircleSharp")]
use leptos::*;
#[cfg(feature = "IoHeartDislikeCircleSharp")]
///This icon requires the feature `IoHeartDislikeCircleSharp` to be enabled.
#[component]
pub fn HeartDislikeCircleSharp(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm63.73,310.36L136.59,176.06l22.74-22.51L342.52,335.91Zm-63.51,4.86c-35.36-25-66.31-51.92-74.91-62.4-20-24.37-29.58-49.4-29.3-76.5a58.27,58.27,0,0,1,.85-9.31l130.21,129.4C279.64,347,266.86,355.86,256.22,363.22Zm74.47-62.4-.31.38L197.33,169a53.8,53.8,0,0,1,10.21-1,59.34,59.34,0,0,1,44.1,19.41L256,192l4.36-4.6A59.34,59.34,0,0,1,304.46,168c30.31,0,55.22,25.27,55.53,56.33C360.27,251.42,350.68,276.45,330.69,300.82Z"
        /> < title > { title } < / title > < / svg >
    }
}
