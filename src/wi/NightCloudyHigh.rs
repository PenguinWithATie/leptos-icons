#[cfg(feature = "WiNightCloudyHigh")]
use leptos::*;
#[cfg(feature = "WiNightCloudyHigh")]
///This icon requires the feature `WiNightCloudyHigh` to be enabled.
#[component]
pub fn NightCloudyHigh(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.58,13.45c0-1.15,0.36-2.18,1.08-3.07C5.38,9.48,6.29,8.9,7.4,8.64c0.31-1.37,1.02-2.49,2.11-3.37s2.35-1.32,3.76-1.32&#xA;	c1.38,0,2.61,0.43,3.69,1.28s1.78,1.95,2.1,3.29h0.33c0.9,0,1.73,0.22,2.49,0.65s1.37,1.03,1.81,1.79c0.44,0.76,0.67,1.58,0.67,2.48&#xA;	c0,0.2-0.01,0.4-0.03,0.61c0.65,0.51,1.16,1.15,1.54,1.91s0.56,1.57,0.56,2.43c0,0.77-0.15,1.5-0.45,2.19&#xA;	c-0.3,0.69-0.7,1.28-1.2,1.78c-0.5,0.49-1.1,0.89-1.79,1.18c-0.69,0.29-1.41,0.44-2.17,0.44c-0.75,0-1.47-0.15-2.16-0.44&#xA;	c-0.69-0.29-1.28-0.69-1.78-1.19c-0.5-0.5-0.89-1.09-1.19-1.78s-0.45-1.41-0.45-2.16H8.38c-1.34-0.06-2.47-0.57-3.4-1.53&#xA;	C4.05,15.94,3.58,14.79,3.58,13.45z M5.29,13.45c0,0.87,0.3,1.62,0.9,2.26c0.6,0.64,1.33,0.98,2.19,1.03h11.19&#xA;	c0.86-0.04,1.59-0.39,2.19-1.03c0.61-0.64,0.91-1.4,0.91-2.26c0-0.88-0.33-1.63-0.98-2.27s-1.42-0.96-2.32-0.96h-1.62&#xA;	c-0.11,0-0.17-0.06-0.17-0.17l-0.07-0.58c-0.11-1.08-0.58-1.99-1.4-2.72s-1.77-1.1-2.86-1.1c-1.09,0-2.05,0.37-2.85,1.1&#xA;	S9.14,8.39,9.04,9.47l-0.08,0.58c0,0.11-0.07,0.17-0.2,0.17H8.24c-0.84,0.1-1.54,0.46-2.1,1.07C5.57,11.9,5.29,12.62,5.29,13.45z&#xA;	 M16.55,18.56c0.06,1.12,0.52,2.07,1.37,2.83c0.85,0.76,1.82,1.14,2.91,1.14c0.6,0,1.17-0.12,1.7-0.35s0.98-0.55,1.34-0.93&#xA;	c0.36-0.39,0.65-0.83,0.85-1.33c0.21-0.5,0.31-1,0.31-1.52c0-0.49-0.1-0.98-0.3-1.47s-0.48-0.94-0.85-1.35&#xA;	c-0.39,0.82-0.97,1.5-1.74,2.02c-0.77,0.52-1.63,0.79-2.57,0.83h-3.03C16.54,18.44,16.54,18.47,16.55,18.56z"
        /> < title > { title } < / title > < / svg >
    }
}
