#[cfg(feature = "FaSolidBacterium")]
use leptos::*;
#[cfg(feature = "FaSolidBacterium")]
///This icon requires the feature `FaSolidBacterium` to be enabled.
#[component]
pub fn Bacterium(
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
        "M423.1 30.6c3.6-12.7-3.7-26-16.5-29.7s-26 3.7-29.7 16.5l-4.2 14.7c-9.8-.4-19.9 .5-29.9 2.8c-12.1 2.8-23.7 5.9-34.9 9.4l-5.9-13.7c-5.2-12.2-19.3-17.8-31.5-12.6s-17.8 19.3-12.6 31.5l4.9 11.3c-22 9.4-42 20.1-60.2 31.8L196 82.7c-7.4-11-22.3-14-33.3-6.7s-14 22.3-6.7 33.3l7.8 11.6c-18 15-33.7 30.8-47.3 47.1L103 157.3c-10.4-8.3-25.5-6.6-33.7 3.7s-6.6 25.5 3.7 33.7l15 12c-2.1 3.2-4.1 6.5-6 9.7c-9.4 15.7-17 31-23.2 45.3l-9.9-3.9c-12.3-4.9-26.3 1.1-31.2 13.4s1.1 26.3 13.4 31.2l11.6 4.6c-.3 1.1-.6 2.1-.9 3.1c-3.5 12.5-5.7 23.2-7.1 31.3c-.7 4.1-1.2 7.5-1.6 10.3c-.2 1.4-.3 2.6-.4 3.6l-.1 1.4-.1 .6 0 .3 0 .1c0 0 0 .1 39.2 3.7l0 0-39.2-3.6c-.5 5-.6 10-.4 14.9l-14.7 4.2C4.7 380.6-2.7 393.8 .9 406.6s16.9 20.1 29.7 16.5l13.8-3.9c10.6 20.7 27.6 37.8 48.5 48.5l-3.9 13.7c-3.6 12.7 3.7 26 16.5 29.7s26-3.7 29.7-16.5l4.2-14.7c23.8 1 46.3-5.5 65.1-17.6L215 473c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-10.6-10.6c9.1-14.1 15.1-30.5 17-48.3l.1-.8c.3-1.7 1-5.1 2.3-9.8l.2-.8 12.6 5.4c12.2 5.2 26.3-.4 31.5-12.6s-.4-26.3-12.6-31.5l-11.3-4.8c9.9-14.9 24.9-31.6 48.6-46l2.1 7.5c3.6 12.7 16.9 20.1 29.7 16.5s20.1-16.9 16.5-29.7L371 259.2c6.9-2.2 14.3-4.3 22.2-6.1c12.9-3 24.7-8 35.2-14.8L439 249c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-10.6-10.6c12.2-19 18.6-41.6 17.6-65.1l14.7-4.2c12.7-3.6 20.1-16.9 16.5-29.7s-16.9-20.1-29.7-16.5l-13.7 3.9c-10.8-21.2-28-38-48.5-48.5l3.9-13.8zM92.1 363.3l0 0L144 368l-51.9-4.7zM112 320a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zM240 184a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"
        /> < title > { title } < / title > < / svg >
    }
}
