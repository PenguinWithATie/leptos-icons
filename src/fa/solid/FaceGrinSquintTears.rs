#[cfg(feature = "FaSolidFaceGrinSquintTears")]
use leptos::*;
#[cfg(feature = "FaSolidFaceGrinSquintTears")]
///This icon requires the feature `FaSolidFaceGrinSquintTears` to be enabled.
#[component]
pub fn FaceGrinSquintTears(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M426.8 14.2C446-5 477.5-4.6 497.1 14.9s20 51 .7 70.3c-6.8 6.8-21.4 12.4-37.4 16.7c-16.3 4.4-34.1 7.5-46.3 9.3c-1.6 .2-3.1 .5-4.6 .6c-4.9 .8-9.1-2.8-9.5-7.4c-.1-.7 0-1.4 .1-2.1c1.6-11.2 4.6-29.6 9-47c.3-1.3 .7-2.6 1-3.9c4.3-15.9 9.8-30.5 16.7-37.4zm-44.7 19c-1.5 4.8-2.9 9.6-4.1 14.3c-4.8 18.9-8 38.5-9.7 50.3c-4 26.8 18.9 49.7 45.7 45.8c11.9-1.6 31.5-4.8 50.4-9.7c4.7-1.2 9.5-2.5 14.3-4.1C534.2 227.5 520.2 353.8 437 437c-83.2 83.2-209.5 97.2-307.2 41.8c1.5-4.8 2.8-9.6 4-14.3c4.8-18.9 8-38.5 9.7-50.3c4-26.8-18.9-49.7-45.7-45.8c-11.9 1.6-31.5 4.8-50.4 9.7c-4.7 1.2-9.5 2.5-14.3 4.1C-22.2 284.5-8.2 158.2 75 75C158.2-8.3 284.5-22.2 382.2 33.2zM51.5 410.1c18.5-5 38.8-8.3 50.9-10c.4-.1 .7-.1 1-.1c5.1-.2 9.2 4.3 8.4 9.6c-1.7 12.1-5 32.4-10 50.9C97.6 476.4 92 491 85.2 497.8C66 517 34.5 516.6 14.9 497.1s-20-51-.7-70.3c6.8-6.8 21.4-12.4 37.4-16.7zM416.9 209c-4.7-11.9-20.8-11-26.8 .3c-19 35.5-45 70.8-77.5 103.3S244.8 371.1 209.3 390c-11.3 6-12.2 22.1-.3 26.8c57.6 22.9 125.8 11 172.3-35.5s58.4-114.8 35.5-172.3zM87.1 285.1c2 2 4.6 3.2 7.3 3.4l56.1 5.1 5.1 56.1c.3 2.8 1.5 5.4 3.4 7.3c6.3 6.3 17.2 3.6 19.8-4.9l29.7-97.4c3.5-11.6-7.3-22.5-19-19L92 265.3c-8.6 2.6-11.3 13.4-4.9 19.8zM265.3 92l-29.7 97.4c-3.5 11.6 7.3 22.5 19 19l97.4-29.7c8.6-2.6 11.3-13.4 4.9-19.8c-2-2-4.6-3.2-7.3-3.4l-56.1-5.1-5.1-56.1c-.2-2.8-1.5-5.4-3.4-7.3c-6.3-6.3-17.2-3.6-19.8 4.9z"
        /> < title > { title } < / title > < / svg >
    }
}
