#[cfg(feature = "TiGlobe")]
use leptos::*;
#[cfg(feature = "TiGlobe")]
///This icon requires the feature `TiGlobe` to be enabled.
#[component]
pub fn Globe(
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
        "M11 20h-4c-.553 0-1 .447-1 1s.447 1 1 1h10c.553 0 1-.447 1-1s-.447-1-1-1h-4v-1.23c1.64-.371 3.146-1.188 4.363-2.406 1.7-1.7 2.637-3.96 2.637-6.364 0-2.067-.692-4.029-1.968-5.619l.675-.673c.391-.391.391-1.023.001-1.415-.391-.391-1.024-.39-1.415-.001l-2.052 2.049.708.708c1.322 1.322 2.051 3.081 2.051 4.951s-.729 3.627-2.051 4.949-3.079 2.051-4.949 2.051-3.627-.729-4.949-2.051c-.391-.391-1.023-.391-1.414 0-.391.39-.391 1.023 0 1.414 1.699 1.7 3.959 2.637 6.363 2.637v1zM11 4c1.657 0 3.157.672 4.243 1.757 1.085 1.086 1.757 2.586 1.757 4.243 0 1.656-.672 3.156-1.757 4.242-1.086 1.086-2.586 1.758-4.243 1.758-1.658 0-3.157-.672-4.242-1.757-1.085-1.086-1.756-2.586-1.756-4.243s.671-3.157 1.756-4.243c1.085-1.085 2.584-1.757 4.242-1.757z"
        /> < title > { title } < / title > < / svg >
    }
}
