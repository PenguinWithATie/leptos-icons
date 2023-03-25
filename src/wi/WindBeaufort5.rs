#[cfg(feature = "WiWindBeaufort5")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort5")]
///This icon requires the feature `WiWindBeaufort5` to be enabled.
#[component]
pub fn WindBeaufort5(
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
        "M4.97,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16&#xA;	c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.28,0.53&#xA;	c0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.56&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C5.03,13.21,4.97,13.34,4.97,13.5z M4.97,11.48c0,0.17,0.06,0.3,0.17,0.39&#xA;	c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26&#xA;	s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51C18,9.09,17.94,9.23,17.94,9.38c0,0.16,0.05,0.3,0.16,0.4&#xA;	c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18&#xA;	c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.56c-0.16,0-0.3,0.06-0.42,0.17&#xA;	C5.03,11.18,4.97,11.32,4.97,11.48z M18.04,19.38c-0.02,0.32,0.01,0.62,0.12,0.91c0.1,0.29,0.27,0.56,0.5,0.81&#xA;	c0.23,0.25,0.55,0.44,0.98,0.59c0.42,0.15,0.92,0.22,1.49,0.22c0.58,0,1.09-0.08,1.53-0.23s0.8-0.34,1.05-0.57&#xA;	c0.25-0.22,0.45-0.49,0.61-0.79c0.16-0.3,0.27-0.57,0.32-0.82c0.05-0.25,0.08-0.49,0.08-0.74c0-0.67-0.21-1.21-0.64-1.61&#xA;	s-0.98-0.61-1.65-0.61c-0.69,0-1.18,0.14-1.45,0.43h-0.02l0.35-1.02h3.45l0.39-1.88h-5.24l-1.45,4.46h2&#xA;	c0.16-0.34,0.53-0.51,1.11-0.51c0.32,0,0.58,0.08,0.77,0.25c0.19,0.17,0.29,0.41,0.29,0.75c0,0.34-0.12,0.61-0.35,0.82&#xA;	c-0.23,0.21-0.57,0.31-1,0.31c-0.31,0-0.56-0.06-0.73-0.17c-0.21-0.11-0.33-0.31-0.36-0.6H18.04z"
        /> < title > { title } < / title > < / svg >
    }
}
