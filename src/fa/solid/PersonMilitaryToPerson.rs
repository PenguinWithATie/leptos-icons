#[cfg(feature = "FaSolidPersonMilitaryToPerson")]
use leptos::*;
#[cfg(feature = "FaSolidPersonMilitaryToPerson")]
///This icon requires the feature `FaSolidPersonMilitaryToPerson` to be enabled.
#[component]
pub fn PersonMilitaryToPerson(
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
        "M67.1 11.7c-8 .9-14.1 7.7-14.1 15.8c0 8.7 7 15.8 15.8 15.9H173.5c8.3-.1 14.9-6.8 14.9-15.1V15.1c0-9-7.8-16-16.7-15L67.1 11.7zM178.6 73.5H62.9c-1.5 5.3-2.4 11-2.4 16.8c0 33.3 27 60.2 60.2 60.2s60.2-27 60.2-60.2c0-5.8-.8-11.5-2.4-16.8zM30.4 240.9v30.1c0 16.6 13.5 30.1 30.1 30.1H180.9c1.7 0 3.3-.1 4.9-.4L50.1 196.4c-12.1 11-19.7 26.9-19.7 44.6zm179.1 40c1.1-3.1 1.6-6.4 1.6-9.9V240.9c0-33.3-27-60.2-60.2-60.2H90.6c-3.5 0-7 .3-10.3 .9l129.1 99.3zM361.6 150.6a60.2 60.2 0 1 0 0-120.5 60.2 60.2 0 1 0 0 120.5zm-30.1 30.1c-33.3 0-60.2 27-60.2 60.2v30.1c0 16.6 13.5 30.1 30.1 30.1H421.9c16.6 0 30.1-13.5 30.1-30.1V240.9c0-33.3-27-60.2-60.2-60.2H331.5zM203.3 423.6c4.9-4.3 7.7-10.5 7.7-17s-2.8-12.7-7.7-17l-60.2-52.7c-6.7-5.8-16.1-7.2-24.2-3.6s-13.3 11.7-13.3 20.6V384l-82.8 0C10.3 384 .2 394.1 .2 406.6s10.1 22.6 22.6 22.6l82.8 0v30.1c0 8.9 5.2 16.9 13.3 20.6s17.5 2.3 24.2-3.6l60.2-52.7zm67.9-17c0 6.5 2.8 12.7 7.7 17l60.2 53.1c6.7 5.9 16.1 7.3 24.2 3.6s13.3-11.7 13.3-20.6l0-30.5 82.8 0c12.5 0 22.6-10.1 22.6-22.6s-10.1-22.6-22.6-22.6l-82.8 0 0-30.1c0-8.9-5.2-16.9-13.3-20.6s-17.5-2.3-24.2 3.6L279 389.6c-4.9 4.3-7.7 10.5-7.7 17z"
        /> < title > { title } < / title > < / svg >
    }
}
