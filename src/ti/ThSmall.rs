#[cfg(feature = "TiThSmall")]
use leptos::*;
#[cfg(feature = "TiThSmall")]
///This icon requires the feature `TiThSmall` to be enabled.
#[component]
pub fn ThSmall(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx
        = "5" cy = "19" r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "5" cy = "12" r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "5"
        cy = "5" r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy =
        "19" r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy =
        "12" r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "5"
        r = "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "19" cy = "19" r =
        "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "19" cy = "12" r =
        "2.5" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "19" cy = "5" r =
        "2.5" /> < title > { title } < / title > < / svg >
    }
}
