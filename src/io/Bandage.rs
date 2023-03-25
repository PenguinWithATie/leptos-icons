#[cfg(feature = "IoBandage")]
use leptos::*;
#[cfg(feature = "IoBandage")]
///This icon requires the feature `IoBandage` to be enabled.
#[component]
pub fn Bandage(
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
        "M275.8,157a16,16,0,0,0-22.63,0l-93.34,93.34a16,16,0,0,0,0,22.63l79.2,79.2h0a16,16,0,0,0,22.63,0L355,258.83a16,16,0,0,0,0-22.63Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M137.21,295.6a47.81,47.81,0,0,1-9.43-13.38L69,341a72.2,72.2,0,0,0,0,102h0a72.37,72.37,0,0,0,102,0l58.77-58.76a47.81,47.81,0,0,1-13.38-9.43Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M392,48a71.55,71.55,0,0,0-51,21l-55.92,55.91a48.05,48.05,0,0,1,13.36,9.45l79.19,79.19a48.05,48.05,0,0,1,9.45,13.36L443,171A72,72,0,0,0,392,48Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M275.8,157a16,16,0,0,0-22.63,0l-93.34,93.34a16,16,0,0,0,0,22.63l79.2,79.2h0a16,16,0,0,0,22.63,0L355,258.83a16,16,0,0,0,0-22.63ZM219.31,267.31a16,16,0,1,1,0-22.62A16,16,0,0,1,219.31,267.31Zm48,48a16,16,0,1,1,0-22.62A16,16,0,0,1,267.31,315.31Zm0-96a16,16,0,1,1,0-22.62A16,16,0,0,1,267.31,219.31Zm48,48a16,16,0,1,1,0-22.62A16,16,0,0,1,315.31,267.31Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M465.61,46.39a104.38,104.38,0,0,0-147.25,0L248.6,116.28a4,4,0,0,0,4.2,6.58,35.74,35.74,0,0,1,11.69-2.54,47.7,47.7,0,0,1,33.94,14.06l79.19,79.19a47.7,47.7,0,0,1,14.06,33.94,35.68,35.68,0,0,1-2.54,11.69,4,4,0,0,0,6.58,4.2l69.89-69.76a104.38,104.38,0,0,0,0-147.25Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M254.34,386.83a47.91,47.91,0,0,1-33.94-14L141.21,293.6a47.81,47.81,0,0,1-9.43-13.38c-4.59-9.7-1.39-25,2.48-36.9a4,4,0,0,0-6.64-4L50.39,316.36A104.12,104.12,0,0,0,197.64,463.61l72.75-72.88a4,4,0,0,0-4.21-6.58C262,385.73,257.78,386.83,254.34,386.83Z"
        /> < title > { title } < / title > < / svg >
    }
}
