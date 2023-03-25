#[cfg(feature = "ImSpotify")]
use leptos::*;
#[cfg(feature = "ImSpotify")]
///This icon requires the feature `ImSpotify` to be enabled.
#[component]
pub fn Spotify(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 0c-4.4 0-8 3.6-8 8s3.6 8 8 8 8-3.6 8-8-3.559-8-8-8zM11.681 11.559c-0.159 0.241-0.441 0.319-0.681 0.159-1.881-1.159-4.241-1.4-7.041-0.759-0.281 0.081-0.519-0.119-0.6-0.359-0.081-0.281 0.119-0.519 0.359-0.6 3.041-0.681 5.681-0.4 7.759 0.881 0.281 0.119 0.322 0.438 0.203 0.678zM12.641 9.359c-0.2 0.281-0.559 0.4-0.841 0.2-2.159-1.319-5.441-1.719-7.959-0.919-0.319 0.081-0.681-0.081-0.759-0.4-0.081-0.319 0.081-0.681 0.4-0.759 2.919-0.881 6.519-0.441 9 1.081 0.238 0.119 0.359 0.519 0.159 0.797zM12.719 7.119c-2.559-1.519-6.841-1.681-9.281-0.919-0.4 0.119-0.8-0.119-0.919-0.481-0.119-0.4 0.119-0.8 0.481-0.919 2.841-0.841 7.519-0.681 10.481 1.081 0.359 0.2 0.481 0.681 0.281 1.041-0.203 0.278-0.681 0.397-1.044 0.197z"
        /> < title > { title } < / title > < / svg >
    }
}
