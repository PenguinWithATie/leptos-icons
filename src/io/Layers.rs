#[cfg(feature = "IoLayers")]
use leptos::*;
#[cfg(feature = "IoLayers")]
///This icon requires the feature `IoLayers` to be enabled.
#[component]
pub fn Layers(
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
        "M256,256c-13.47,0-26.94-2.39-37.44-7.17l-148-67.49C63.79,178.26,48,169.25,48,152.24s15.79-26,22.58-29.12L219.86,55.05c20.57-9.4,51.61-9.4,72.19,0l149.37,68.07c6.79,3.09,22.58,12.1,22.58,29.12s-15.79,26-22.58,29.11l-148,67.48C282.94,253.61,269.47,256,256,256ZM432.76,155.14h0Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M441.36,226.81,426.27,220,387.5,237.74l-94,43c-10.5,4.8-24,7.19-37.44,7.19s-26.93-2.39-37.42-7.19l-94.07-43L85.79,220l-15.22,6.84C63.79,229.93,48,239,48,256s15.79,26.08,22.56,29.17l148,67.63C229,357.6,242.49,360,256,360s26.94-2.4,37.44-7.19L441.31,285.2C448.12,282.11,464,273.09,464,256S448.23,229.93,441.36,226.81Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M441.36,330.8,426.27,324,387.5,341.73l-94,42.95c-10.5,4.78-24,7.18-37.44,7.18s-26.93-2.39-37.42-7.18l-94.07-43L85.79,324l-15.22,6.84C63.79,333.93,48,343,48,360s15.79,26.07,22.56,29.15l148,67.59C229,461.52,242.54,464,256,464s26.88-2.48,37.38-7.27l147.92-67.57C448.12,386.08,464,377.06,464,360S448.23,333.93,441.36,330.8Z"
        /> < title > { title } < / title > < / svg >
    }
}
