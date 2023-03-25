#[cfg(feature = "WiWindBeaufort3")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort3")]
///This icon requires the feature `WiWindBeaufort3` to be enabled.
#[component]
pub fn WindBeaufort3(
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
        "M5.03,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16&#xA;	c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53&#xA;	s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.62&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C5.09,13.21,5.03,13.34,5.03,13.5z M5.03,11.48c0,0.17,0.06,0.3,0.17,0.39&#xA;	c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26&#xA;	s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51C18.06,9.09,18,9.23,18,9.38c0,0.16,0.05,0.3,0.16,0.4&#xA;	c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18&#xA;	c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.62c-0.16,0-0.3,0.06-0.42,0.17&#xA;	C5.09,11.18,5.03,11.32,5.03,11.48z M18.12,19.52c0,0.27,0.05,0.53,0.16,0.79c0.11,0.26,0.27,0.5,0.5,0.75&#xA;	c0.23,0.24,0.55,0.43,0.96,0.58s0.9,0.22,1.46,0.22c1.21,0,2.08-0.24,2.63-0.72c0.55-0.48,0.82-1.13,0.82-1.95&#xA;	c0-0.36-0.1-0.69-0.3-0.99c-0.2-0.3-0.47-0.47-0.79-0.51v-0.02c0.43-0.08,0.79-0.27,1.07-0.58c0.28-0.31,0.43-0.69,0.43-1.12&#xA;	c0-0.31-0.06-0.58-0.17-0.82c-0.11-0.24-0.26-0.43-0.44-0.58c-0.18-0.15-0.39-0.27-0.64-0.37c-0.25-0.1-0.5-0.16-0.75-0.2&#xA;	c-0.25-0.04-0.52-0.06-0.8-0.06c-0.92,0-1.68,0.22-2.28,0.67c-0.59,0.45-0.96,1.12-1.1,2.01h2.03c0.04-0.31,0.17-0.55,0.38-0.72&#xA;	c0.21-0.17,0.47-0.26,0.78-0.26c0.29,0,0.51,0.06,0.68,0.18S23,16.11,23,16.32c0,0.47-0.42,0.7-1.27,0.7h-0.47l-0.29,1.4h0.44&#xA;	c0.68,0,1.02,0.23,1.02,0.7c0,0.31-0.11,0.55-0.34,0.72c-0.23,0.17-0.5,0.25-0.83,0.25c-0.38,0-0.66-0.11-0.83-0.34&#xA;	c-0.17-0.21-0.24-0.51-0.21-0.89h-2.07C18.13,19.06,18.12,19.27,18.12,19.52z"
        /> < title > { title } < / title > < / svg >
    }
}
