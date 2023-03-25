#[cfg(feature = "FaSolidPersonMilitaryRifle")]
use leptos::*;
#[cfg(feature = "FaSolidPersonMilitaryRifle")]
///This icon requires the feature `FaSolidPersonMilitaryRifle` to be enabled.
#[component]
pub fn PersonMilitaryRifle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M128 39c0-13 10-23.8 22.9-24.9L302.7 1.4C312 .7 320 8 320 17.4V48c0 8.8-7.2 16-16 16H153c-13.8 0-25-11.2-25-25zm17.6 57H302.4c1 5.2 1.6 10.5 1.6 16c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-5.5 .6-10.8 1.6-16zm228 364.3L320 369.7V480c0 1.3-.1 2.5-.2 3.8L145.5 234.9c16.6-7.1 34.6-10.9 53.3-10.9h50.4c15.9 0 31.3 2.8 45.8 7.9L389.9 67.7c-7.7-4.4-10.3-14.2-5.9-21.9s14.2-10.3 21.9-5.9l13.9 8 13.9 8c7.7 4.4 10.3 14.2 5.9 21.9L384 173.9l1.6 .9c15.3 8.8 20.6 28.4 11.7 43.7L360.6 282c2 2.8 3.9 5.8 5.7 8.8l76.1 128.8c11.2 19 4.9 43.5-14.1 54.8s-43.5 4.9-54.8-14.1zM288 512H160c-17.7 0-32-14.3-32-32V369.7L74.4 460.3c-11.2 19-35.8 25.3-54.8 14.1S-5.7 438.7 5.6 419.7L81.7 290.8c9.4-15.8 21.7-29.3 36-40L299.1 510c-3.5 1.3-7.2 2-11.1 2zM264 320a24 24 0 1 0 0-48 24 24 0 1 0 0 48z"
        /> < title > { title } < / title > < / svg >
    }
}
