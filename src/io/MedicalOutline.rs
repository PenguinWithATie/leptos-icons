#[cfg(feature = "IoMedicalOutline")]
use leptos::*;
#[cfg(feature = "IoMedicalOutline")]
///This icon requires the feature `IoMedicalOutline` to be enabled.
#[component]
pub fn MedicalOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M429.93,174.27l-16.47-28.59a15.49,15.49,0,0,0-21.15-5.7l-98.39,57a4,4,0,0,1-6-3.5L288,80a16,16,0,0,0-16-16H240a16,16,0,0,0-16,16l.07,113.57a4,4,0,0,1-6,3.5l-98.39-57a15.49,15.49,0,0,0-21.15,5.7L82.07,174.37a15.42,15.42,0,0,0,5.69,21.1l98.49,57.08a4,4,0,0,1,0,6.9L87.76,316.53a15.54,15.54,0,0,0-5.69,21.1l16.47,28.59a15.49,15.49,0,0,0,21.15,5.7l98.39-57a4,4,0,0,1,6,3.5L224,432a16,16,0,0,0,16,16h32a16,16,0,0,0,16-16l-.07-113.67a4,4,0,0,1,6-3.5l98.39,57a15.49,15.49,0,0,0,21.15-5.7l16.47-28.59a15.42,15.42,0,0,0-5.69-21.1l-98.49-57.08a4,4,0,0,1,0-6.9l98.49-57.08A15.51,15.51,0,0,0,429.93,174.27Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
