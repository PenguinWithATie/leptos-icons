#[cfg(feature = "AiOutlinedAudioMuted")]
use leptos::*;
#[cfg(feature = "AiOutlinedAudioMuted")]
///This icon requires the feature `AiOutlinedAudioMuted` to be enabled.
#[component]
pub fn AudioMuted(
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
        stroke_witdh = "0" style = style t = "1569682811064" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "8066" width = "200" height = "200"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < defs
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" >< style type = "text/css" /></ defs >< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M682 455V311l-76 76v68c-0.1 50.7-42 92.1-94 92-19.1 0.1-36.8-5.4-52-15l-54 55c29.1 22.4 65.9 36 106 36 93.8 0 170-75.1 170-168z"
        p - id = "8067" />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M833 446h-60c-4.4 0-8 3.6-8 8 0 140.3-113.7 254-254 254-63 0-120.7-23-165-61l-54 54c48.9 43.2 110.8 72.3 179 81v102H326c-13.9 0-24.9 14.3-25 32v36c0.1 4.4 2.9 8 6 8h408c3.2 0 6-3.6 6-8v-36c0-17.7-11-32-25-32H547V782c165.3-17.9 294-157.9 294-328 0-4.4-3.6-8-8-8zM846.1 68.3l-43.5-41.9c-3.1-3-8.1-3-11.2 0.1l-129 129C634.3 101.2 577 64 511 64c-93.9 0-170 75.3-170 168v224c0 6.7 0.4 13.3 1.2 19.8l-68 68c-10.5-27.9-16.3-58.2-16.2-89.8-0.2-4.4-3.8-8-8-8h-60c-4.4 0-8 3.6-8 8 0 53 12.5 103 34.6 147.4l-137 137c-3.1 3.1-3.1 8.2 0 11.3l42.7 42.7c3.1 3.1 8.2 3.1 11.3 0L846.2 79.8l0.1-0.1c3.1-3.2 3-8.3-0.2-11.4zM417 401V232c0-50.6 41.9-92 94-92 46 0 84.1 32.3 92.3 74.7L417 401z"
        p - id = "8068" /> < title > { title } < / title > < / svg >
    }
}
