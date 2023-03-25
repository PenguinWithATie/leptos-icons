#[cfg(feature = "VsPreserveCase")]
use leptos::*;
#[cfg(feature = "VsPreserveCase")]
///This icon requires the feature `VsPreserveCase` to be enabled.
#[component]
pub fn PreserveCase(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.51358 11H7.51456L6.69815 8.84082H3.43253L2.66446 11H1.66006L4.61417 3.29785H5.54874L8.51358 11ZM6.40274 8.02979L5.19424 4.74805C5.15486 4.64062 5.11547 4.46875 5.07608 4.23242H5.0546C5.01879 4.45085 4.97761 4.62272 4.93106 4.74805L3.73331 8.02979H6.40274Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.59725 11V3.29785H11.7887C12.4547 3.29785 12.9828 3.46077 13.3731 3.78662C13.7634 4.11247 13.9586 4.53678 13.9586 5.05957C13.9586 5.49642 13.8404 5.87598 13.6041 6.19824C13.3678 6.52051 13.0419 6.74967 12.6265 6.88574V6.90723C13.1458 6.9681 13.5611 7.16504 13.8726 7.49805C14.1842 7.82747 14.3399 8.25716 14.3399 8.78711C14.3399 9.44596 14.1036 9.97949 13.6309 10.3877C13.1583 10.7959 12.5621 11 11.8424 11H9.59725ZM10.4996 4.11426V6.60107H11.4234C11.9176 6.60107 12.3061 6.48291 12.589 6.24658C12.8718 6.00667 13.0133 5.67008 13.0133 5.23682C13.0133 4.48844 12.5209 4.11426 11.5362 4.11426H10.4996ZM10.4996 7.41211V10.1836H11.7242C12.2542 10.1836 12.6641 10.0583 12.9542 9.80762C13.2478 9.55697 13.3946 9.21322 13.3946 8.77637C13.3946 7.86686 12.7751 7.41211 11.5362 7.41211H10.4996Z"
        /> < title > { title } < / title > < / svg >
    }
}
