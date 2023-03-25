#[cfg(feature = "WiMoonWaningCrescent4")]
use leptos::*;
#[cfg(feature = "WiMoonWaningCrescent4")]
///This icon requires the feature `WiMoonWaningCrescent4` to be enabled.
#[component]
pub fn MoonWaningCrescent4(
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
        "M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.1,4.1s3.61,1.51,5.65,1.51c-2.07-1.01-3.59-2.45-4.56-4.33&#xA;	S9,17.19,9,14.44c0-2.53,0.56-4.78,1.69-6.75s2.57-3.47,4.31-4.5c-2.04,0-3.93,0.5-5.65,1.51s-3.1,2.37-4.1,4.09&#xA;	S3.74,12.4,3.74,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
