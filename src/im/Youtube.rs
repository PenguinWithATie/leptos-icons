#[cfg(feature = "ImYoutube")]
use leptos::*;
#[cfg(feature = "ImYoutube")]
///This icon requires the feature `ImYoutube` to be enabled.
#[component]
pub fn Youtube(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.841 4.8c0 0-0.156-1.103-0.637-1.587-0.609-0.637-1.291-0.641-1.603-0.678-2.237-0.163-5.597-0.163-5.597-0.163h-0.006c0 0-3.359 0-5.597 0.163-0.313 0.038-0.994 0.041-1.603 0.678-0.481 0.484-0.634 1.587-0.634 1.587s-0.159 1.294-0.159 2.591v1.213c0 1.294 0.159 2.591 0.159 2.591s0.156 1.103 0.634 1.588c0.609 0.637 1.409 0.616 1.766 0.684 1.281 0.122 5.441 0.159 5.441 0.159s3.363-0.006 5.6-0.166c0.313-0.037 0.994-0.041 1.603-0.678 0.481-0.484 0.637-1.588 0.637-1.588s0.159-1.294 0.159-2.591v-1.213c-0.003-1.294-0.162-2.591-0.162-2.591zM6.347 10.075v-4.497l4.322 2.256-4.322 2.241z"
        /> < title > { title } < / title > < / svg >
    }
}
