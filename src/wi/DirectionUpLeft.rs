#[cfg(feature = "WiDirectionUpLeft")]
use leptos::*;
#[cfg(feature = "WiDirectionUpLeft")]
///This icon requires the feature `WiDirectionUpLeft` to be enabled.
#[component]
pub fn DirectionUpLeft(
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
        "M10.03,14.31V9.84c0-0.3,0.1-0.55,0.3-0.75s0.45-0.3,0.74-0.3h4.48c0.29,0,0.54,0.1,0.74,0.3s0.3,0.45,0.3,0.75&#xA;	c0,0.29-0.1,0.53-0.3,0.73s-0.45,0.29-0.74,0.29h-1.95l6.06,6.06c0.18,0.21,0.26,0.46,0.26,0.78c0,0.29-0.09,0.53-0.26,0.72&#xA;	c-0.2,0.19-0.46,0.28-0.79,0.28c-0.3,0-0.55-0.09-0.73-0.28l-6.02-6.05v1.95c0,0.3-0.1,0.55-0.3,0.75c-0.2,0.2-0.45,0.3-0.75,0.3&#xA;	c-0.29,0-0.54-0.1-0.74-0.31S10.03,14.6,10.03,14.31z"
        /> < title > { title } < / title > < / svg >
    }
}
