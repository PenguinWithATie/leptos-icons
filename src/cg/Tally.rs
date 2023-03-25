#[cfg(feature = "CgTally")]
use leptos::*;
#[cfg(feature = "CgTally")]
///This icon requires the feature `CgTally` to be enabled.
#[component]
pub fn Tally(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.66124 2.67117C3.71341 2.4 3.97956 2.18018 4.2557 2.18018H6.2557C6.53184 2.18018 6.71341 2.4 6.66124 2.67117L5.33877 9.54511C5.2866 9.81628 5.02045 10.0361 4.74431 10.0361H2.74431C2.46817 10.0361 2.2866 9.81628 2.33877 9.54511L3.66124 2.67117Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.66124 2.67117C8.71341 2.4 8.97956 2.18018 9.2557 2.18018H11.2557C11.5318 2.18018 11.7134 2.4 11.6612 2.67117L8.07168 21.329C8.01951 21.6002 7.75336 21.82 7.47722 21.82H5.47722C5.20108 21.82 5.01951 21.6002 5.07168 21.329L8.66124 2.67117Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.6612 2.67117C13.7134 2.4 13.9796 2.18018 14.2557 2.18018H16.2557C16.5318 2.18018 16.7134 2.4 16.6612 2.67117L13.0717 21.329C13.0195 21.6002 12.7534 21.82 12.4772 21.82H10.4772C10.2011 21.82 10.0195 21.6002 10.0717 21.329L13.6612 2.67117Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.6612 2.67117C18.7134 2.4 18.9796 2.18018 19.2557 2.18018H21.2557C21.5318 2.18018 21.7134 2.4 21.6612 2.67117L20.3388 9.54511C20.2866 9.81628 20.0205 10.0361 19.7443 10.0361H17.7443C17.4682 10.0361 17.2866 9.81628 17.3388 9.54511L18.6612 2.67117Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
