#[cfg(feature = "TiPlug")]
use leptos::*;
#[cfg(feature = "TiPlug")]
///This icon requires the feature `TiPlug` to be enabled.
#[component]
pub fn Plug(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18,6h-1V3c0-0.6-0.4-1-1-1h-2c-0.6,0-1,0.4-1,1v3h-2V3c0-0.6-0.4-1-1-1H8C7.4,2,7,2.4,7,3v3H6C5.4,6,5,6.4,5,7v4&#xD;&#xA;	c0,0.1,0,0.1,0,0.2c0.2,2.5,1.8,4.6,4,5.6V20c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2v-3.2c2.2-1,3.7-3.1,4-5.6c0-0.1,0-0.1,0-0.2V7&#xD;&#xA;	C19,6.4,18.6,6,18,6z M14,3h2v3h-2V3z M8,3h2v3H8V3z M13,20h-2v-2h2V20z M12,15.5c-2.2,0-4.1-1.5-4.7-3.5h9.5&#xD;&#xA;	C16.1,14,14.2,15.5,12,15.5z M17,10.5c0,0.2,0,0.3-0.1,0.5H7.1C7,10.8,7,10.7,7,10.5V8h10V10.5z"
        /> < title > { title } < / title > < / svg >
    }
}
