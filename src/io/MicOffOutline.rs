#[cfg(feature = "IoMicOffOutline")]
use leptos::*;
#[cfg(feature = "IoMicOffOutline")]
///This icon requires the feature `IoMicOffOutline` to be enabled.
#[component]
pub fn MicOffOutline(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "432" y1 = "400" x2 = "96" y2 = "64" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,240V208.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,368,208v32a111.58,111.58,0,0,1-2.45,23.31,4.05,4.05,0,0,0,1.07,3.69l21.82,21.81a2,2,0,0,0,3.29-.72A143.27,143.27,0,0,0,400,240Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,352A112.36,112.36,0,0,1,144,240V208.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,112,208v32c0,74,56.1,135.12,128,143.11V432H192.45c-8.61,0-16,6.62-16.43,15.23A16,16,0,0,0,192,464H319.55c8.61,0,16-6.62,16.43-15.23A16,16,0,0,0,320,432H272V383.11a143.08,143.08,0,0,0,52-16.22,4,4,0,0,0,.91-6.35L307,342.63a4,4,0,0,0-4.51-.78A110.78,110.78,0,0,1,256,352Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,80a47.18,47.18,0,0,1,48,48v74.72a4,4,0,0,0,1.17,2.82L332.59,233a2,2,0,0,0,3.41-1.42V128.91C336,85,301,48.6,257.14,48a79.66,79.66,0,0,0-68.47,36.57,4,4,0,0,0,.54,5l19.54,19.54a2,2,0,0,0,3.25-.63A47.44,47.44,0,0,1,256,80Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M207.27,242.9,179.41,215a2,2,0,0,0-3.41,1.42V239a80.89,80.89,0,0,0,23.45,56.9,78.55,78.55,0,0,0,77.8,21.19,2,2,0,0,0,.86-3.35L253.2,288.83a4.08,4.08,0,0,0-2.42-1.15c-21.65-2.52-39.48-20.44-42.37-42.43A4,4,0,0,0,207.27,242.9Z"
        /> < title > { title } < / title > < / svg >
    }
}
