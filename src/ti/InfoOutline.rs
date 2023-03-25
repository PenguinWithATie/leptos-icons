#[cfg(feature = "TiInfoOutline")]
use leptos::*;
#[cfg(feature = "TiInfoOutline")]
///This icon requires the feature `TiInfoOutline` to be enabled.
#[component]
pub fn InfoOutline(
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
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12,5.51c0.56,0,1.12,0.35,1.54,1.06l5.91,9.85C20.31,17.84,19.65,19,18,19H6c-1.65,0-2.31-1.16-1.46-2.57l5.91-9.85&#xD;&#xA;	C10.88,5.86,11.44,5.51,12,5.51 M12,3.51c-1.3,0-2.48,0.74-3.26,2.03L2.83,15.4c-0.44,0.74-0.66,1.5-0.66,2.23&#xD;&#xA;	c0,0.56,0.14,1.11,0.42,1.6C3.23,20.35,4.47,21,6,21h12c1.53,0,2.77-0.65,3.41-1.77c0.29-0.51,0.43-1.07,0.42-1.65&#xD;&#xA;	c-0.01-0.71-0.23-1.46-0.66-2.18l-5.91-9.85C14.48,4.25,13.3,3.51,12,3.51z M13.5,16.75c0,0-0.71,0.36-1.07,0.18&#xD;&#xA;	c-0.36-0.18-0.43-0.54-0.23-1.15l0.41-1.22c0.4-1.22-0.12-2.08-1.08-2.13c-1.26-0.07-2.02,0.83-2.02,0.83s0.71-0.36,1.07-0.18&#xD;&#xA;	c0.36,0.18,0.43,0.54,0.23,1.15l-0.41,1.22c-0.4,1.22,0.12,2.07,1.08,2.13C12.74,17.64,13.5,16.75,13.5,16.75z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "10" r = "1.3" />
        < title > { title } < / title > < / svg >
    }
}
