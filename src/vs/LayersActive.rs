#[cfg(feature = "VsLayersActive")]
use leptos::*;
#[cfg(feature = "VsLayersActive")]
///This icon requires the feature `VsLayersActive` to be enabled.
#[component]
pub fn LayersActive(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M8.18535 1.08325L7.62706 1.08717L1.71796 5.12422L1.72152 5.95233L7.63062 9.91528L8.1818 9.91912L14.2727 5.95617L14.2762 5.1203L8.18535 1.08325ZM2.89198 5.53323L7.91335 2.10268L13.0891 5.5332L7.91329 8.90079L2.89198 5.53323ZM7.63059 12.4153L1.79257 8.5H3.58794L7.91326 11.4008L12.3716 8.5H14.2053L13.4056 9.02031C13.2722 9.00688 13.1369 9 13 9C11.224 9 9.71839 10.1574 9.19622 11.7591L8.18177 12.4191L7.63059 12.4153ZM9.00447 13.1908L7.91326 13.9008L3.58794 11H1.79257L7.63059 14.9153L8.18177 14.9191L9.20113 14.2559C9.08965 13.9185 9.02187 13.5612 9.00447 13.1908Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d =
        "M11.3333 10.5056C11.8266 10.1759 12.4067 10 13 10C13.7954 10.001 14.5578 10.3174 15.1202 10.8798C15.6826 11.4422 15.999 12.2046 16 13C16 13.5933 15.8241 14.1734 15.4944 14.6667C15.1648 15.1601 14.6962 15.5446 14.1481 15.7716C13.5999 15.9987 12.9967 16.0581 12.4147 15.9424C11.8328 15.8266 11.2982 15.5409 10.8787 15.1213C10.4591 14.7018 10.1734 14.1672 10.0576 13.5853C9.94189 13.0033 10.0013 12.4001 10.2284 11.8519C10.4554 11.3038 10.8399 10.8352 11.3333 10.5056ZM13.0315 14.3226L14.8213 11.9363L14.0213 11.3363L12.541 13.3099L11.6655 12.6095L11.0408 13.3903L12.3192 14.413L13.0315 14.3226Z"
        /> < title > { title } < / title > < / svg >
    }
}
