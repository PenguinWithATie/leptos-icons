#[cfg(feature = "HiMdSolidEnvelopeOpen")]
use leptos::*;
#[cfg(feature = "HiMdSolidEnvelopeOpen")]
///This icon requires the feature `HiMdSolidEnvelopeOpen` to be enabled.
#[component]
pub fn EnvelopeOpen(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2.10557 6.44747C1.428 6.78626 1 7.47878 1 8.23633V16.0003C1 17.1048 1.89543 18.0003 3 18.0003H17C18.1046 18.0003 19 17.1048 19 16.0003V8.23633C19 7.47878 18.572 6.78626 17.8944 6.44747L10.8944 2.94747C10.3314 2.66595 9.66863 2.66595 9.10557 2.94747L2.10557 6.44747ZM3.58541 10.4544C3.21493 10.2692 2.76442 10.4194 2.57918 10.7899C2.39394 11.1603 2.54411 11.6108 2.91459 11.7961L8.77016 14.7239C9.54436 15.111 10.4556 15.111 11.2298 14.7239L17.0823 11.7976C17.4528 11.6124 17.6029 11.1619 17.4177 10.7914C17.2325 10.4209 16.7819 10.2708 16.4115 10.456L10.559 13.3822C10.2071 13.5582 9.79289 13.5582 9.44098 13.3822L3.58541 10.4544Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
