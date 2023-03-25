#[cfg(feature = "SiWindicss")]
use leptos::*;
#[cfg(feature = "SiWindicss")]
///This icon requires the feature `SiWindicss` to be enabled.
#[component]
pub fn Windicss(
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
        "M4.12 15.561H.96v2.431h3.16zm13.728 4.22A4.224 4.224 0 0113.628 24c-2.452 0-3.918-1.838-3.918-3.617h2.412c0 .5.467 1.206 1.506 1.206a1.81 1.81 0 001.809-1.809c0-.997-.917-1.808-2.563-1.808H5.589v-2.411h7.285c3.097 0 4.974 1.893 4.974 4.22zA4.224 4.224 0 0113.628 24c-2.452 0-3.918-1.838-3.918-3.617h2.412c0 .5.467 1.206 1.506 1.206a1.81 1.81 0 001.809-1.809c0-.997-.917-1.808-2.563-1.808H5.589v-2.411h7.285c3.097 0 4.974 1.893 4.974 4.22zM18.48 3.72c-2.66 0-4.536 2.022-4.536 4.682h2.136c0-1.322.96-2.282 2.4-2.282s2.16.96 2.16 2.282-.935 2.411-3.48 2.411H.96v2.411h16.56c3.769 0 5.52-2.422 5.52-4.822 0-2.66-1.8-4.682-4.56-4.682zm-5.981.5A4.224 4.224 0 008.279 0C5.827 0 4.361 1.838 4.361 3.617h2.411c0-.5.468-1.206 1.507-1.206a1.81 1.81 0 011.809 1.809c0 .997-.728 1.808-2.563 1.808H.96v2.411h6.565c3.097 0 4.974-1.893 4.974-4.22zA4.224 4.224 0 008.279 0C5.827 0 4.361 1.838 4.361 3.617h2.411c0-.5.468-1.206 1.507-1.206a1.81 1.81 0 011.809 1.809c0 .997-.728 1.808-2.563 1.808H.96v2.411h6.565c3.097 0 4.974-1.893 4.974-4.22z"
        /> < title > { title } < / title > < / svg >
    }
}
