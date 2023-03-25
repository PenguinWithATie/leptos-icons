#[cfg(feature = "FaSolidHandsBound")]
use leptos::*;
#[cfg(feature = "FaSolidHandsBound")]
///This icon requires the feature `FaSolidHandsBound` to be enabled.
#[component]
pub fn HandsBound(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 32C64 14.3 49.7 0 32 0S0 14.3 0 32V96v59.1 .7V192v21.9c0 14.2 5.1 27.9 14.3 38.7L99.6 352H96c-13.3 0-24 10.7-24 24s10.7 24 24 24h32H256h64H448h32c13.3 0 24-10.7 24-24s-10.7-24-24-24h-3.6l85.3-99.5c9.2-10.8 14.3-24.5 14.3-38.7V192 155.8v-.7V96 32c0-17.7-14.3-32-32-32s-32 14.3-32 32V96v48.8l-69.3 92.4c-5.7 7.6-16.1 9.6-24.2 4.8c-9.7-5.7-12.1-18.7-5.1-27.5L441 180c10.8-13.5 8.9-33.3-4.4-44.5s-33-9.8-44.5 3.2l-46.7 52.5C329 209.7 320 233.4 320 258.1V320v32H256V320 258.1c0-24.6-9-48.4-25.4-66.8l-46.7-52.5c-11.5-13-31.3-14.4-44.5-3.2s-15.2 30.9-4.4 44.5l27.6 34.5c7 8.8 4.7 21.8-5.1 27.5c-8.1 4.8-18.6 2.7-24.2-4.8L64 144.8V96 32zm64 448v32H256V480h64v32H448V480h32c13.3 0 24-10.7 24-24s-10.7-24-24-24H448 320 256 128 96c-13.3 0-24 10.7-24 24s10.7 24 24 24h32z"
        /> < title > { title } < / title > < / svg >
    }
}
