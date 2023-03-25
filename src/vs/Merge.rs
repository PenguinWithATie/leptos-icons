#[cfg(feature = "VsMerge")]
use leptos::*;
#[cfg(feature = "VsMerge")]
///This icon requires the feature `VsMerge` to be enabled.
#[component]
pub fn Merge(
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
        "M10.5 4.646L8.354 2.5h-.707L5.5 4.646l.707.707L7.3 4.261V5.28h-.02v.456l.025.001.006.319c.004.187.02.379.05.574.03.195.069.39.117.586.048.195.114.404.2.627.155.379.343.722.565 1.031.221.309.46.598.715.867.255.27.508.535.76.797.25.262.478.541.681.838.203.297.368.621.494.973.125.351.188.755.188 1.213v.884H12.5v-.884a5.991 5.991 0 0 0-.166-1.39 4.638 4.638 0 0 0-.427-1.1 5.875 5.875 0 0 0-.604-.897c-.222-.27-.453-.527-.693-.774-.24-.246-.471-.492-.693-.738a6.39 6.39 0 0 1-.604-.785 3.794 3.794 0 0 1-.433-.914 3.676 3.676 0 0 1-.16-1.13V5.28h-.001v-1l1.074 1.074.707-.708zM7.042 9.741a8.19 8.19 0 0 0 .329-.369 6.06 6.06 0 0 1-.62-1.15L6.744 8.2a7.26 7.26 0 0 1-.095-.263c-.17.256-.359.498-.565.726-.222.246-.453.492-.693.738-.24.247-.47.504-.693.774-.221.27-.423.568-.604.896a4.643 4.643 0 0 0-.427 1.102 5.995 5.995 0 0 0-.166 1.389v.884h1.42v-.884c0-.457.062-.862.188-1.213.125-.352.29-.676.493-.973.203-.297.43-.576.682-.838.251-.262.504-.527.76-.797z"
        /> < title > { title } < / title > < / svg >
    }
}
