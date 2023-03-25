#[cfg(feature = "IoChatbubbles")]
use leptos::*;
#[cfg(feature = "IoChatbubbles")]
///This icon requires the feature `IoChatbubbles` to be enabled.
#[component]
pub fn Chatbubbles(
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
        "M60.44,389.17c0,.07,0,.2-.08.38C60.39,389.43,60.41,389.3,60.44,389.17Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M439.9,405.6a26.77,26.77,0,0,1-9.59-2l-56.78-20.13-.42-.17a9.88,9.88,0,0,0-3.91-.76,10.32,10.32,0,0,0-3.62.66c-1.38.52-13.81,5.19-26.85,8.77-7.07,1.94-31.68,8.27-51.43,8.27-50.48,0-97.68-19.4-132.89-54.63A183.38,183.38,0,0,1,100.3,215.1a175.9,175.9,0,0,1,4.06-37.58c8.79-40.62,32.07-77.57,65.55-104A194.76,194.76,0,0,1,290.3,32c52.21,0,100.86,20,137,56.18,34.16,34.27,52.88,79.33,52.73,126.87a177.86,177.86,0,0,1-30.3,99.15l-.19.28-.74,1c-.17.23-.34.45-.5.68l-.15.27a21.63,21.63,0,0,0-1.08,2.09l15.74,55.94a26.42,26.42,0,0,1,1.12,7.11A24,24,0,0,1,439.9,405.6Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M299.87,425.39a15.74,15.74,0,0,0-10.29-8.1c-5.78-1.53-12.52-1.27-17.67-1.65a201.78,201.78,0,0,1-128.82-58.75A199.21,199.21,0,0,1,86.4,244.16C85,234.42,85,232,85,232a16,16,0,0,0-28-10.58h0S49.12,230,45.4,238.61a162.09,162.09,0,0,0,11,150.06C59,393,59,395,58.42,399.5c-2.73,14.11-7.51,39-10,51.91a24,24,0,0,0,8,22.92l.46.39A24.34,24.34,0,0,0,72,480a23.42,23.42,0,0,0,9-1.79l53.51-20.65a8.05,8.05,0,0,1,5.72,0c21.07,7.84,43,12,63.78,12a176,176,0,0,0,74.91-16.66c5.46-2.56,14-5.34,19-11.12A15,15,0,0,0,299.87,425.39Z"
        /> < title > { title } < / title > < / svg >
    }
}
