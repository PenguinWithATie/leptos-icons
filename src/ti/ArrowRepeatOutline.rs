#[cfg(feature = "TiArrowRepeatOutline")]
use leptos::*;
#[cfg(feature = "TiArrowRepeatOutline")]
///This icon requires the feature `TiArrowRepeatOutline` to be enabled.
#[component]
pub fn ArrowRepeatOutline(
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
        "M18.994 7.187l.006-.187c0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-2.892 2.891c-.53-.473-1.221-.77-1.987-.77h-1c-3.859 0-7 3.14-7 7 0 3.306 2.14 6.084 5.006 6.813l-.006.187c0 .801.312 1.555.879 2.121.566.567 1.32.879 2.121.879s1.555-.312 2.121-.879l2.892-2.891c.53.473 1.221.77 1.987.77h1c3.859 0 7-3.14 7-7 0-3.306-2.14-6.084-5.006-6.813zm-1.994 11.813h-1c-.553 0-1-.448-1-1s.447-1 1-1h1c1.654 0 3-1.346 3-3s-1.121-3-2.5-3h-2.086l1.293 1.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.293 1.293h2.086c2.481 0 4.5 2.243 4.5 5s-2.243 5-5 5zm.749-6.971c.7.164 1.251 1 1.251 1.971 0 1.103-.897 2-2 2h-1c-.857 0-1.584.544-1.868 1.304l-3.718-3.718c-.378-.378-.88-.586-1.414-.586s-1.036.208-1.414.586-.586.88-.586 1.414c0 .345.087.677.251.971-.7-.164-1.251-1-1.251-1.971 0-1.103.897-2 2-2h1c.857 0 1.584-.544 1.868-1.304l3.718 3.718c.378.378.88.586 1.414.586s1.036-.208 1.414-.586.586-.88.586-1.414c0-.345-.087-.677-.251-.971zm-7.749-2.029c0 .552-.447 1-1 1h-1c-1.654 0-3 1.346-3 3s1.121 3 2.5 3h2.086l-1.293-1.293c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l3.707 3.707-3.707 3.707c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l1.293-1.293h-2.086c-2.481 0-4.5-2.243-4.5-5s2.243-5 5-5h1c.553 0 1 .448 1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
