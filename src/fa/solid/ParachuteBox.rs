#[cfg(feature = "FaSolidParachuteBox")]
use leptos::*;
#[cfg(feature = "FaSolidParachuteBox")]
///This icon requires the feature `FaSolidParachuteBox` to be enabled.
#[component]
pub fn ParachuteBox(
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
        "M380.5 192c.3-5.3 .5-10.6 .5-16c0-51-15.9-96-40.2-127.6C316.5 16.9 285.2 0 253 0s-63.5 16.9-87.8 48.4C140.9 80 125 125 125 176c0 5.4 .2 10.7 .5 16H237V320H205c-7 0-13.7 1.5-19.7 4.2L65.2 192H93.5c-.3-5.3-.5-10.6-.5-16c0-64 22.2-121.2 57.1-159.3C59 49.3 15.6 122.6 1.2 173.6C-1.5 183.1 6 192 15.9 192h6L162.2 346.3c-3.3 6.5-5.2 13.9-5.2 21.7v96c0 26.5 21.5 48 48 48h96c26.5 0 48-21.5 48-48V368c0-7.8-1.9-15.2-5.2-21.7L484.1 192h6c9.9 0 17.4-8.9 14.7-18.4C490.4 122.6 447 49.3 355.9 16.7C390.8 54.8 413 112.1 413 176c0 5.4-.2 10.7-.5 16h28.3L320.7 324.2c-6-2.7-12.7-4.2-19.7-4.2H269V192H380.5z"
        /> < title > { title } < / title > < / svg >
    }
}
