#[cfg(feature = "IoHammerOutline")]
use leptos::*;
#[cfg(feature = "IoHammerOutline")]
///This icon requires the feature `IoHammerOutline` to be enabled.
#[component]
pub fn HammerOutline(
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
        "M277.42,247a24.68,24.68,0,0,0-4.08-5.47L255,223.44a21.63,21.63,0,0,0-6.56-4.57,20.93,20.93,0,0,0-23.28,4.27c-6.36,6.26-18,17.68-39,38.43C146,301.3,71.43,367.89,37.71,396.29a16,16,0,0,0-1.09,23.54l39,39.43a16.13,16.13,0,0,0,23.67-.89c29.24-34.37,96.3-109,136-148.23,20.39-20.06,31.82-31.58,38.29-37.94A21.76,21.76,0,0,0,277.42,247Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M478.43,201l-34.31-34a5.44,5.44,0,0,0-4-1.59,5.59,5.59,0,0,0-4,1.59h0a11.41,11.41,0,0,1-9.55,3.27c-4.48-.49-9.25-1.88-12.33-4.86-7-6.86,1.09-20.36-5.07-29a242.88,242.88,0,0,0-23.08-26.72c-7.06-7-34.81-33.47-81.55-52.53a123.79,123.79,0,0,0-47-9.24c-26.35,0-46.61,11.76-54,18.51-5.88,5.32-12,13.77-12,13.77A91.29,91.29,0,0,1,202.35,77a79.53,79.53,0,0,1,23.28-1.49C241.19,76.8,259.94,84.1,270,92c16.21,13,23.18,30.39,24.27,52.83.8,16.69-15.23,37.76-30.44,54.94a7.85,7.85,0,0,0,.4,10.83l21.24,21.23a8,8,0,0,0,11.14.1c13.93-13.51,31.09-28.47,40.82-34.46s17.58-7.68,21.35-8.09A35.71,35.71,0,0,1,380.08,194a13.65,13.65,0,0,1,3.08,2.38c6.46,6.56,6.07,17.28-.5,23.74l-2,1.89a5.5,5.5,0,0,0,0,7.84l34.31,34a5.5,5.5,0,0,0,4,1.58,5.65,5.65,0,0,0,4-1.58L478.43,209A5.82,5.82,0,0,0,478.43,201Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
