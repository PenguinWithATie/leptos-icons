#[cfg(feature = "SiEslint")]
use leptos::*;
#[cfg(feature = "SiEslint")]
///This icon requires the feature `SiEslint` to be enabled.
#[component]
pub fn Eslint(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.257 9.132L11.816 6.5a.369.369 0 0 1 .368 0l4.559 2.632a.369.369 0 0 1 .184.32v5.263a.37.37 0 0 1-.184.319l-4.559 2.632a.369.369 0 0 1-.368 0l-4.559-2.632a.369.369 0 0 1-.184-.32V9.452a.37.37 0 0 1 .184-.32M23.852 11.53l-5.446-9.475c-.198-.343-.564-.596-.96-.596H6.555c-.396 0-.762.253-.96.596L.149 11.509a1.127 1.127 0 0 0 0 1.117l5.447 9.398c.197.342.563.517.959.517h10.893c.395 0 .76-.17.959-.512l5.446-9.413a1.069 1.069 0 0 0 0-1.086m-4.51 4.556a.4.4 0 0 1-.204.338L12.2 20.426a.395.395 0 0 1-.392 0l-6.943-4.002a.4.4 0 0 1-.205-.338V8.08c0-.14.083-.269.204-.338L11.8 3.74c.12-.07.272-.07.392 0l6.943 4.003a.4.4 0 0 1 .206.338z"
        /> < title > { title } < / title > < / svg >
    }
}
