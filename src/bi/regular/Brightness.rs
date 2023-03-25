#[cfg(feature = "BiRegularBrightness")]
use leptos::*;
#[cfg(feature = "BiRegularBrightness")]
///This icon requires the feature `BiRegularBrightness` to be enabled.
#[component]
pub fn Brightness(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19.707 9.293 19 8.586V6a1 1 0 0 0-1-1h-2.586l-.707-.707-2-2a.999.999 0 0 0-1.414 0l-2 2L8.586 5H6a1 1 0 0 0-1 1v2.586l-.707.707-2 2a.999.999 0 0 0 0 1.414l2 2 .707.707V18a1 1 0 0 0 1 1h2.586l.707.707 2 2a.997.997 0 0 0 1.414 0l2-2 .707-.707H18a1 1 0 0 0 1-1v-2.586l.707-.707 2-2a.999.999 0 0 0 0-1.414l-2-2zm-2.414 5-.293.293V17h-2.414l-.293.293-1 1L12 19.586l-1.293-1.293-1-1L9.414 17H7v-2.414l-.293-.293-1-1L4.414 12l1.293-1.293 1-1L7 9.414V7h2.414l.293-.293 1-1L12 4.414l1.293 1.293 1 1 .293.293H17v2.414l.293.293 1 1L19.586 12l-1.293 1.293-1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 8c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4z" /> < title > {
        title } < / title > < / svg >
    }
}
