#[cfg(feature = "FaSolidPersonRifle")]
use leptos::*;
#[cfg(feature = "FaSolidPersonRifle")]
///This icon requires the feature `FaSolidPersonRifle` to be enabled.
#[component]
pub fn PersonRifle(
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
        "M249.2 192c25.4 0 49.8 7.1 70.8 19.9V512H128V337.7L74.4 428.3c-11.2 19-35.8 25.3-54.8 14.1S-5.7 406.7 5.6 387.7L81.7 258.8c24.5-41.4 69-66.8 117.1-66.8h50.4zM144 80a80 80 0 1 1 160 0A80 80 0 1 1 144 80zM432 0c8.8 0 16 7.2 16 16V132.3c9.6 5.5 16 15.9 16 27.7V269.3l16-5.3V208c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v84.5c0 6.9-4.4 13-10.9 15.2L464 325.3V352h48c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H468l23 92.1c2.5 10.1-5.1 19.9-15.5 19.9H416c-8.8 0-16-7.2-16-16V400H384c-17.7 0-32-14.3-32-32V224c0-17.7 14.3-32 32-32V160c0-11.8 6.4-22.2 16-27.7V32c-8.8 0-16-7.2-16-16s7.2-16 16-16h16 16z"
        /> < title > { title } < / title > < / svg >
    }
}
