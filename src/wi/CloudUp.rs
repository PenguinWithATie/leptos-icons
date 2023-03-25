#[cfg(feature = "WiCloudUp")]
use leptos::*;
#[cfg(feature = "WiCloudUp")]
///This icon requires the feature `WiCloudUp` to be enabled.
#[component]
pub fn CloudUp(
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
        "M4.64,16.88c0,1.33,0.46,2.48,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.33&#xA;	c0-0.12-0.06-0.19-0.17-0.19c-0.86-0.04-1.58-0.38-2.18-1.02s-0.9-1.39-0.9-2.25c0-0.82,0.28-1.54,0.84-2.16&#xA;	c0.56-0.61,1.26-0.97,2.1-1.07h0.53c0.13,0,0.2-0.06,0.2-0.18l0.06-0.57c0.11-1.08,0.57-1.99,1.38-2.72s1.77-1.1,2.86-1.1&#xA;	c1.08,0,2.03,0.37,2.85,1.1c0.82,0.73,1.28,1.64,1.39,2.72l0.08,0.57c0,0.12,0.06,0.18,0.18,0.18h1.61c0.89,0,1.66,0.32,2.31,0.96&#xA;	s0.98,1.4,0.98,2.26c0,0.86-0.3,1.61-0.9,2.25c-0.6,0.64-1.33,0.98-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.19v1.33&#xA;	c0,0.11,0.07,0.17,0.2,0.17c0.87-0.02,1.67-0.26,2.4-0.71c0.73-0.45,1.31-1.05,1.73-1.8c0.42-0.75,0.63-1.57,0.63-2.44&#xA;	c0-0.67-0.13-1.31-0.39-1.91c-0.26-0.61-0.62-1.13-1.06-1.57c-0.44-0.44-0.97-0.79-1.58-1.05c-0.61-0.26-1.25-0.39-1.92-0.39h-0.32&#xA;	c-0.33-1.34-1.03-2.43-2.11-3.29c-1.07-0.85-2.3-1.28-3.68-1.28c-1.41,0-2.67,0.44-3.76,1.32s-1.79,2-2.1,3.36&#xA;	c-1.11,0.26-2.02,0.83-2.73,1.73C4.99,14.71,4.64,15.73,4.64,16.88z M11.58,17.51c0,0.25,0.08,0.46,0.24,0.64&#xA;	c0.15,0.15,0.35,0.23,0.61,0.23c0.24,0,0.45-0.08,0.62-0.23l1.11-1.14v3.98c0,0.24,0.08,0.44,0.23,0.61&#xA;	c0.16,0.17,0.35,0.25,0.59,0.25c0.23,0,0.43-0.08,0.6-0.25c0.17-0.17,0.25-0.37,0.25-0.61v-3.94l1.12,1.11&#xA;	c0.4,0.31,0.81,0.31,1.22,0c0.16-0.15,0.24-0.36,0.24-0.62c0-0.24-0.08-0.44-0.24-0.62l-2.59-2.57c-0.16-0.16-0.36-0.24-0.6-0.24&#xA;	c-0.24,0-0.44,0.08-0.59,0.24l-2.58,2.57C11.66,17.08,11.58,17.27,11.58,17.51z"
        /> < title > { title } < / title > < / svg >
    }
}
