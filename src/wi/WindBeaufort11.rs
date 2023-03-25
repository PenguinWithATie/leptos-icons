#[cfg(feature = "WiWindBeaufort11")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort11")]
///This icon requires the feature `WiWindBeaufort11` to be enabled.
#[component]
pub fn WindBeaufort11(
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
        "M4.68,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21&#xA;	c-0.13-0.11-0.26-0.16-0.4-0.16c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41&#xA;	c0.36,0.36,0.78,0.53,1.28,0.53s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27&#xA;	c-0.35-0.35-0.77-0.53-1.26-0.53H5.27c-0.16,0-0.3,0.06-0.42,0.17C4.74,13.21,4.68,13.34,4.68,13.5z M4.68,11.48&#xA;	c0,0.17,0.06,0.3,0.17,0.39c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27&#xA;	c0-0.49-0.17-0.91-0.52-1.26s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42&#xA;	c0,0.16,0.05,0.3,0.16,0.4c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18&#xA;	c0.17,0,0.33,0.06,0.46,0.18c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.27&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C4.74,11.18,4.68,11.32,4.68,11.48z M17.57,21.9h2.47l1.65-7.99h-2.47L17.57,21.9z M21.3,21.9h2.46&#xA;	l1.65-7.99h-2.45L21.3,21.9z"
        /> < title > { title } < / title > < / svg >
    }
}
