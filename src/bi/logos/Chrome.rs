#[cfg(feature = "BiLogosChrome")]
use leptos::*;
#[cfg(feature = "BiLogosChrome")]
///This icon requires the feature `BiLogosChrome` to be enabled.
#[component]
pub fn Chrome(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10.742 2.04c-1.404.183-3.06.808-4.281 1.626-1.01.664-2.397 2.02-2.309 2.251.193.501 3.28 5.658 3.33 5.562.038-.067.095-.279.123-.49.25-1.385 1.425-2.704 2.897-3.253.568-.221.683-.221 5.495-.27l4.917-.047-.395-.646c-1.385-2.26-3.522-3.819-6.197-4.512-.731-.193-2.81-.318-3.58-.22z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.2 7.217c-.453.799-.983 2.415-1.107 3.358-.588 4.273 1.568 8.4 5.379 10.315.894.452 2.174.885 2.732.933l.356.029 1.674-2.838c.915-1.559 1.655-2.849 1.636-2.868-.02-.019-.231.039-.481.125-1.569.53-3.387.086-4.57-1.116-.424-.424-1.002-1.357-2.84-4.542C4.71 8.41 3.642 6.601 3.603 6.601c-.028 0-.211.279-.403.616z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.15 8.804c1.222 1.242 1.655 3.003 1.116 4.59-.086.26-1.212 2.271-2.501 4.485-1.29 2.203-2.349 4.031-2.349 4.06 0 .115 1.328.057 2.175-.087 4.32-.74 7.573-4.002 8.265-8.276.26-1.558.164-2.925-.307-4.503l-.25-.837h-6.707l.557.568z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.608 8.563C9.598 8.987 8.905 9.7 8.53 10.71c-.173.453-.202.713-.173 1.424.03.75.068.963.347 1.511.366.75.962 1.329 1.751 1.703.462.221.654.25 1.54.25.895 0 1.077-.029 1.559-.26.712-.326 1.462-1.077 1.79-1.79.23-.48.259-.663.259-1.558 0-.886-.029-1.078-.25-1.54-.375-.788-.952-1.386-1.703-1.75-.568-.28-.742-.318-1.56-.348-.788-.019-.99.01-1.48.212z"
        /> < title > { title } < / title > < / svg >
    }
}
