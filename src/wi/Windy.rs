#[cfg(feature = "WiWindy")]
use leptos::*;
#[cfg(feature = "WiWindy")]
///This icon requires the feature `WiWindy` to be enabled.
#[component]
pub fn Windy(
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
        "M4.65,15.5c0-0.22,0.08-0.41,0.23-0.56c0.16-0.15,0.35-0.22,0.57-0.22h12.08c0.22,0,0.4,0.07,0.54,0.22&#xA;	c0.14,0.15,0.22,0.34,0.22,0.57c0,0.22-0.07,0.4-0.22,0.54c-0.14,0.14-0.32,0.22-0.54,0.22H5.45c-0.22,0-0.42-0.07-0.57-0.22&#xA;	C4.72,15.9,4.65,15.72,4.65,15.5z M7.06,12.6c0-0.22,0.08-0.4,0.23-0.55c0.15-0.15,0.34-0.23,0.56-0.23h12.09&#xA;	c0.21,0,0.39,0.08,0.54,0.23c0.15,0.15,0.22,0.33,0.22,0.55c0,0.22-0.07,0.4-0.22,0.56c-0.15,0.15-0.33,0.23-0.54,0.23H7.86&#xA;	c-0.22,0-0.41-0.08-0.56-0.23S7.06,12.82,7.06,12.6z M8.68,18.34c0-0.21,0.08-0.39,0.24-0.54c0.14-0.14,0.32-0.22,0.54-0.22h12.1&#xA;	c0.22,0,0.41,0.07,0.56,0.22c0.15,0.14,0.22,0.32,0.22,0.54s-0.08,0.41-0.23,0.56s-0.34,0.23-0.56,0.23H9.46&#xA;	c-0.22,0-0.4-0.08-0.56-0.23S8.68,18.56,8.68,18.34z M19.26,15.5c0-0.23,0.07-0.42,0.22-0.57c0.15-0.15,0.34-0.22,0.57-0.22h4.52&#xA;	c0.23,0,0.42,0.07,0.57,0.22c0.15,0.15,0.22,0.34,0.22,0.56c0,0.22-0.07,0.4-0.22,0.54c-0.15,0.14-0.34,0.22-0.56,0.22h-4.52&#xA;	c-0.23,0-0.42-0.07-0.57-0.22C19.33,15.9,19.26,15.72,19.26,15.5z"
        /> < title > { title } < / title > < / svg >
    }
}
