#[cfg(feature = "WiMoonrise")]
use leptos::*;
#[cfg(feature = "WiMoonrise")]
///This icon requires the feature `WiMoonrise` to be enabled.
#[component]
pub fn Moonrise(
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
        "M7.8,14.86c0-0.98,0.19-1.92,0.58-2.82c0.38-0.9,0.9-1.67,1.55-2.32c0.65-0.65,1.43-1.17,2.32-1.56s1.84-0.58,2.83-0.58&#xA;	h1.17c0.16,0.04,0.24,0.14,0.24,0.28l0.04,0.9c0.04,1.3,0.51,2.41,1.41,3.33s2,1.41,3.28,1.46l0.85,0.07c0.16,0,0.23,0.08,0.23,0.23&#xA;	v1.01c0.01,1.03-0.19,2-0.58,2.92h-2.05c0.51-0.74,0.83-1.59,0.97-2.53c-1.67-0.35-3.02-1.07-4.03-2.16s-1.59-2.37-1.75-3.83&#xA;	c-0.97,0.05-1.85,0.35-2.66,0.9c-0.8,0.55-1.43,1.24-1.87,2.08c-0.44,0.84-0.66,1.72-0.66,2.63c0,1.07,0.28,2.04,0.83,2.92H8.4&#xA;	C8,16.85,7.8,15.88,7.8,14.86z M8.09,20.87c0-0.29,0.09-0.52,0.28-0.68c0.18-0.18,0.41-0.26,0.69-0.26h2.63L14.8,17&#xA;	c0.1-0.08,0.22-0.08,0.35,0l3.16,2.92h2.77c0.27,0,0.5,0.09,0.69,0.28c0.19,0.18,0.29,0.41,0.29,0.67c0,0.27-0.1,0.5-0.29,0.69&#xA;	c-0.19,0.19-0.42,0.29-0.69,0.29h-3.38c-0.1,0-0.2-0.02-0.29-0.07l-2.41-2.27l-2.39,2.27c-0.08,0.05-0.17,0.07-0.28,0.07H9.06&#xA;	c-0.27,0-0.5-0.1-0.69-0.29S8.09,21.14,8.09,20.87z"
        /> < title > { title } < / title > < / svg >
    }
}
