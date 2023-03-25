#[cfg(feature = "IoBonfire")]
use leptos::*;
#[cfg(feature = "IoBonfire")]
///This icon requires the feature `IoBonfire` to be enabled.
#[component]
pub fn Bonfire(
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
        "M273.38,368.37c-.81-9.23-8.86-16.44-18.55-16.44A18.63,18.63,0,0,0,236.63,366l-18.2,88.36a35.59,35.59,0,0,0-.93,7.87c0,19.93,16.68,33.77,37.33,33.77s37.34-13.84,37.34-33.77a36.16,36.16,0,0,0-1.29-9.45Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M411.05,407.89a42.66,42.66,0,0,0-5.95-4.36L335.57,355c-6.77-4.24-14-4.13-19.25,1a13.52,13.52,0,0,0-2,17.19l52.5,69a38,38,0,0,0,4,4.69c9.1,10.16,29.28,10.72,40.37,0C422.13,435.92,422,416.49,411.05,407.89Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M463.3,335.93H392.88c-4.55,0-8.88,3.35-8.88,8.15s2.95,7.85,6.92,9.16l66.43,20.55C467,376,480,367.44,480,356.71,480,343.08,472.4,335.93,463.3,335.93Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M128.22,344.08c0-4.95-4.55-8.15-9.45-8.15H48.35c-8.87,0-16.35,9.58-16.35,20.31S44.62,376,54.3,373.79l67.43-20.55C126.12,351.93,128.22,349,128.22,344.08Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M176.55,355.05,107,403.41a32.29,32.29,0,0,0-6,4.34,26.33,26.33,0,0,0,0,38.56,29.41,29.41,0,0,0,40.36,0,30.75,30.75,0,0,0,4-4.68L197.9,373c3.5-5.57,2.92-12.48-2-17A15,15,0,0,0,176.55,355.05Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M293.46,242.39c10-5.42,19.45-10.54,28.31-16.16,42.46-26.92,62.23-59,62.23-101,0-64.66-56.07-104.4-108.82-109-6.49-.57-15-.42-19.91,3.88s-5.88,12.56-6.15,19.1c-1.38,33.61-28.38,59-57,85.86-28,26.3-56.93,53.49-62.71,91C121,270.75,151.15,302.78,169.87,317a31.88,31.88,0,0,0,19.3,6.51,32.32,32.32,0,0,0,7.41-.87,31.75,31.75,0,0,0,21.27-16.95C232.76,275.3,263.62,258.57,293.46,242.39Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M139.82,156.57c12.22-15.76,27-29.68,41.37-43.15C201.11,94.7,219,77.84,227.69,59.56c-12.24-7.37-27.36-11.36-46.51-11.36-17.79,0-20.39,5.18-20.39,19.06,0,12.56-6.53,20.54-14.34,30.65C137.8,109.1,128,121.77,128,140.84c0,10.23,1.29,18.77,4.2,26.37Q135.7,161.89,139.82,156.57Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M330.34,239.74c-9.33,5.92-19,11.16-29.25,16.71-28.91,15.68-56.21,30.48-68.88,56.28-.64,1.32-1.25,2.5-1.88,3.61a8,8,0,0,0,3.89,11.3c12.31,5.1,25.13,8.27,38.91,8.27a111.42,111.42,0,0,0,78.24-31.37A107.45,107.45,0,0,0,384,226.85a86.56,86.56,0,0,0-1.33-15,8,8,0,0,0-13.8-4C358.69,219.32,345.94,229.85,330.34,239.74Z"
        /> < title > { title } < / title > < / svg >
    }
}
