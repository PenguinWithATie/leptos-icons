#[cfg(feature = "RiOthersLineBell")]
use leptos::*;
#[cfg(feature = "RiOthersLineBell")]
///This icon requires the feature `RiOthersLineBell` to be enabled.
#[component]
pub fn Bell(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M14.121 9.879c4.296 4.295 6.829 8.728 5.657 9.9-.475.474-1.486.34-2.807-.273a9.008 9.008 0 0 1-10.59-.474l-.038.04-1.414-1.415.038-.04A9.006 9.006 0 0 1 4.495 7.03c-.614-1.322-.748-2.333-.273-2.808 1.128-1.128 5.277 1.177 9.417 5.182l.482.475zm-1.414 1.414C10.823 9.409 8.87 7.842 7.236 6.87l-.186.18a7.002 7.002 0 0 0-.657 9.142l1.846-1.846a2 2 0 1 1 1.416 1.415l-1.848 1.846a7.002 7.002 0 0 0 9.143-.657l.179-.188-.053-.089c-.976-1.615-2.52-3.53-4.369-5.38zm7.071-7.071a2 2 0 0 1-.164 2.976 9.015 9.015 0 0 1 .662 8.345 21.168 21.168 0 0 0-1.386-2.306 6.99 6.99 0 0 0-1.94-6.187 6.992 6.992 0 0 0-6.187-1.94 21.092 21.092 0 0 0-2.306-1.386 9.016 9.016 0 0 1 8.347.663 2 2 0 0 1 2.974-.165z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
