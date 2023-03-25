#[cfg(feature = "WiNightCloudyWindy")]
use leptos::*;
#[cfg(feature = "WiNightCloudyWindy")]
///This icon requires the feature `WiNightCloudyWindy` to be enabled.
#[component]
pub fn NightCloudyWindy(
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
        "M2.43,21c0,0.25,0.09,0.45,0.27,0.6c0.17,0.17,0.37,0.26,0.61,0.26h9.54c0.23,0,0.43-0.08,0.59-0.25&#xA;	c0.16-0.17,0.24-0.37,0.24-0.61s-0.08-0.44-0.24-0.61c-0.16-0.17-0.35-0.25-0.59-0.25H3.31c-0.24,0-0.44,0.09-0.62,0.26&#xA;	C2.52,20.57,2.43,20.77,2.43,21z M5.07,17.97c0,0.23,0.09,0.42,0.27,0.58c0.16,0.16,0.36,0.24,0.6,0.24h9.55&#xA;	c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.24-0.08-0.44-0.24-0.6c-0.16-0.17-0.35-0.25-0.59-0.25H5.94&#xA;	c-0.24,0-0.44,0.08-0.61,0.25C5.15,17.54,5.07,17.74,5.07,17.97z M6.21,15.64c0,0.07,0.07,0.11,0.2,0.11h1.42&#xA;	c0.09,0,0.16-0.05,0.23-0.14c0.22-0.54,0.57-0.99,1.05-1.35c0.47-0.36,1-0.56,1.58-0.6l0.54-0.07c0.12,0,0.18-0.06,0.18-0.18&#xA;	l0.07-0.51c0.11-1.08,0.57-1.99,1.38-2.72c0.81-0.73,1.77-1.1,2.87-1.1s2.06,0.36,2.87,1.09c0.81,0.72,1.28,1.63,1.39,2.73&#xA;	l0.08,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.69,0.32,2.33,0.95c0.64,0.63,0.96,1.39,0.96,2.29c0,0.89-0.32,1.65-0.96,2.29&#xA;	c-0.64,0.64-1.42,0.96-2.33,0.96h-6.91c-0.11,0-0.17,0.06-0.17,0.17v1.38c0,0.12,0.06,0.18,0.17,0.18h6.91&#xA;	c0.91,0,1.74-0.22,2.51-0.67c0.77-0.44,1.37-1.05,1.82-1.81c0.45-0.76,0.67-1.59,0.67-2.49c0-0.71-0.15-1.37-0.44-2.01&#xA;	c0.77-1.01,1.15-2.1,1.15-3.29c0-0.95-0.24-1.83-0.71-2.64s-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.7-2.64-0.7&#xA;	c-1.52,0-2.81,0.56-3.84,1.67C17.6,7.6,16.7,7.4,15.74,7.4c-0.93,0-1.81,0.2-2.63,0.59s-1.51,0.95-2.07,1.66&#xA;	c-0.56,0.71-0.94,1.52-1.13,2.43c-0.88,0.2-1.64,0.6-2.29,1.2c-0.65,0.6-1.11,1.33-1.36,2.17L6.21,15.64z M6.83,24.09&#xA;	c0,0.23,0.09,0.43,0.26,0.58c0.16,0.16,0.36,0.24,0.6,0.24h9.56c0.24,0,0.44-0.08,0.6-0.23s0.25-0.35,0.25-0.59&#xA;	s-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.25-0.6-0.25H7.69c-0.23,0-0.43,0.09-0.6,0.26C6.92,23.66,6.83,23.86,6.83,24.09z&#xA;	 M19.83,9.02c0.67-0.65,1.5-0.98,2.47-0.98c0.99,0,1.83,0.35,2.52,1.04c0.69,0.69,1.04,1.53,1.04,2.52c0,0.63-0.16,1.22-0.49,1.77&#xA;	c-0.98-0.96-2.15-1.43-3.52-1.43h-0.32C21.3,10.84,20.73,9.87,19.83,9.02z"
        /> < title > { title } < / title > < / svg >
    }
}
