#[cfg(feature = "FaSolidSolarPanel")]
use leptos::*;
#[cfg(feature = "FaSolidSolarPanel")]
///This icon requires the feature `FaSolidSolarPanel` to be enabled.
#[component]
pub fn SolarPanel(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M115.2 0C84.7 0 58.5 21.5 52.5 51.4L1.3 307.4C-6.6 347 23.6 384 64 384H281v64H217c-17.7 0-32 14.3-32 32s14.3 32 32 32H409c17.7 0 32-14.3 32-32s-14.3-32-32-32H345V384H562c40.4 0 70.7-36.9 62.8-76.6l-51.2-256C567.5 21.5 541.3 0 510.8 0H115.2zM253.9 64H372.1l10.4 104h-139L253.9 64zM195.3 168H94.4L115.2 64h90.4L195.3 168zM84.8 216H190.5L180.1 320H64L84.8 216zm153.9 0H387.3l10.4 104-169.4 0 10.4-104zm196.8 0H541.2L562 320h-116L435.5 216zm96-48H430.7L420.3 64h90.4l31.4-6.3L510.8 64l20.8 104z"
        /> < title > { title } < / title > < / svg >
    }
}
