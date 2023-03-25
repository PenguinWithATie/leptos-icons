#[cfg(feature = "BiRegularDialpadAlt")]
use leptos::*;
#[cfg(feature = "BiRegularDialpadAlt")]
///This icon requires the feature `BiRegularDialpadAlt` to be enabled.
#[component]
pub fn DialpadAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "6" r = "2" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "6" cy = "6" r = "2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "18" cy = "6" r = "2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "6" cy = "12" r = "2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "18" cy = "12" r = "2" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "12" cy = "18" r = "2" /> < title > { title } <
        / title > < / svg >
    }
}
