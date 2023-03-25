#[cfg(feature = "WiHurricane")]
use leptos::*;
#[cfg(feature = "WiHurricane")]
///This icon requires the feature `WiHurricane` to be enabled.
#[component]
pub fn Hurricane(
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
        "M11.08,14.53v-0.02c-0.01-0.08,0-0.2,0-0.37c0.01-0.16,0.04-0.43,0.1-0.81c0.06-0.38,0.14-0.76,0.25-1.15&#xA;	s0.28-0.84,0.51-1.35c0.23-0.51,0.5-0.99,0.82-1.44C13.08,8.94,13.5,8.47,14.02,8c0.52-0.47,1.1-0.89,1.73-1.24&#xA;	c0.16-0.09,0.32-0.11,0.49-0.06s0.3,0.15,0.38,0.31c0.09,0.16,0.11,0.32,0.06,0.5c-0.05,0.17-0.15,0.31-0.3,0.39&#xA;	c-1.31,0.72-2.32,1.73-3.03,3.05c0.54-0.25,1.08-0.38,1.63-0.38c1.07,0,2,0.38,2.77,1.15c0.77,0.77,1.15,1.69,1.15,2.76&#xA;	c0,0.08,0,0.16,0,0.24c0,0.08-0.02,0.25-0.04,0.52c-0.02,0.27-0.06,0.52-0.11,0.77c-0.05,0.25-0.13,0.56-0.23,0.93&#xA;	c-0.11,0.37-0.23,0.73-0.38,1.06c-0.15,0.33-0.34,0.7-0.58,1.1s-0.51,0.77-0.81,1.12c-0.3,0.35-0.66,0.7-1.08,1.05&#xA;	c-0.43,0.35-0.89,0.67-1.39,0.95c-0.09,0.06-0.2,0.08-0.31,0.08c-0.26,0-0.45-0.12-0.58-0.35c-0.09-0.16-0.11-0.32-0.06-0.49&#xA;	c0.05-0.17,0.15-0.3,0.31-0.38c1.34-0.75,2.36-1.78,3.06-3.08c-0.54,0.26-1.11,0.38-1.71,0.38c-0.69,0-1.34-0.17-1.94-0.52&#xA;	c-0.6-0.34-1.07-0.81-1.43-1.41C11.27,15.87,11.09,15.23,11.08,14.53z M12.78,14.48c0,0.61,0.22,1.13,0.65,1.57&#xA;	c0.43,0.43,0.95,0.65,1.56,0.65c0.57,0,1.06-0.19,1.49-0.57c0.42-0.38,0.66-0.85,0.73-1.41l0.01-0.23c0-0.02,0-0.04,0.01-0.05&#xA;	c-0.01-0.6-0.23-1.11-0.66-1.52c-0.43-0.42-0.96-0.62-1.57-0.62c-0.56,0-1.04,0.18-1.46,0.54s-0.66,0.82-0.73,1.36l-0.02,0.25V14.48&#xA;	z"
        /> < title > { title } < / title > < / svg >
    }
}
