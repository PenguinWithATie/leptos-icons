#[cfg(feature = "BiRegularBookReader")]
use leptos::*;
#[cfg(feature = "BiRegularBookReader")]
///This icon requires the feature `BiRegularBookReader` to be enabled.
#[component]
pub fn BookReader(
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
        "M21 8c-.202 0-4.85.029-9 2.008C7.85 8.029 3.202 8 3 8a1 1 0 0 0-1 1v9.883a1 1 0 0 0 .305.719c.195.188.48.305.729.28l.127-.001c.683 0 4.296.098 8.416 2.025.016.008.034.005.05.011.119.049.244.083.373.083s.254-.034.374-.083c.016-.006.034-.003.05-.011 4.12-1.928 7.733-2.025 8.416-2.025l.127.001c.238.025.533-.092.729-.28.194-.189.304-.449.304-.719V9a1 1 0 0 0-1-1zM4 10.049c1.485.111 4.381.48 7 1.692v7.742c-3-1.175-5.59-1.494-7-1.576v-7.858zm16 7.858c-1.41.082-4 .401-7 1.576v-7.742c2.619-1.212 5.515-1.581 7-1.692v7.858z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "5" r = "3" /> <
        title > { title } < / title > < / svg >
    }
}
