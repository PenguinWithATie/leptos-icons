#[cfg(feature = "WiMoonWaningGibbous3")]
use leptos::*;
#[cfg(feature = "WiMoonWaningGibbous3")]
///This icon requires the feature `WiMoonWaningGibbous3` to be enabled.
#[component]
pub fn MoonWaningGibbous3(
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
        "M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56&#xA;	c3.61-2.07,5.42-5.81,5.42-11.22c0-1.31-0.15-2.56-0.45-3.74s-0.71-2.24-1.23-3.17s-1.1-1.75-1.72-2.46s-1.3-1.33-2.02-1.86&#xA;	c-1.52,0-2.98,0.3-4.37,0.89s-2.58,1.39-3.58,2.4s-1.8,2.2-2.39,3.59S3.74,12.96,3.74,14.49z"
        /> < title > { title } < / title > < / svg >
    }
}
