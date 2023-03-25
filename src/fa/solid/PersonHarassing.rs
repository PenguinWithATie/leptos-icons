#[cfg(feature = "FaSolidPersonHarassing")]
use leptos::*;
#[cfg(feature = "FaSolidPersonHarassing")]
///This icon requires the feature `FaSolidPersonHarassing` to be enabled.
#[component]
pub fn PersonHarassing(
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
        "M192 96a48 48 0 1 0 0-96 48 48 0 1 0 0 96zM59.4 304.5L88 256.9V480c0 17.7 14.3 32 32 32s32-14.3 32-32V352h16V480c0 17.7 14.3 32 32 32s32-14.3 32-32V235.3l47.4 57.1c11.3 13.6 31.5 15.5 45.1 4.2s15.5-31.5 4.2-45.1l-73.7-88.9c-18.2-22-45.3-34.7-73.9-34.7H145.1c-33.7 0-64.9 17.7-82.3 46.6l-58.3 97c-9.1 15.1-4.2 34.8 10.9 43.9s34.8 4.2 43.9-10.9zM480 240a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM464 344v58.7l-41.4-41.4c-7.3-7.3-17.6-10.6-27.8-9s-18.9 8.1-23.5 17.3l-48 96c-7.9 15.8-1.5 35 14.3 42.9s35 1.5 42.9-14.3L408.8 438l54.7 54.7c12.4 12.4 29.1 19.3 46.6 19.3c36.4 0 65.9-29.5 65.9-65.9V344c0-30.9-25.1-56-56-56s-56 25.1-56 56zM288 48c0 8.8 7.2 16 16 16h56c8.8 0 16-7.2 16-16s-7.2-16-16-16H304c-8.8 0-16 7.2-16 16zm-.8 49.7c-7.9-4-17.5-.7-21.5 7.2s-.7 17.5 7.2 21.5l48 24c7.9 4 17.5 .7 21.5-7.2s.7-17.5-7.2-21.5l-48-24z"
        /> < title > { title } < / title > < / svg >
    }
}
