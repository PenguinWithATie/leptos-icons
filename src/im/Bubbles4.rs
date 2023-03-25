#[cfg(feature = "ImBubbles4")]
use leptos::*;
#[cfg(feature = "ImBubbles4")]
///This icon requires the feature `ImBubbles4` to be enabled.
#[component]
pub fn Bubbles4(
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
        stroke_witdh = "0" style = style version = "1.1" width = "18" height = "16"
        viewBox = "0 0 18 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M7.5 2c-0.792 0-1.556 0.124-2.272 0.369-0.671 0.23-1.267 0.554-1.773 0.963-0.938 0.759-1.455 1.731-1.455 2.737 0 0.562 0.157 1.109 0.467 1.623 0.323 0.537 0.811 1.028 1.41 1.421 0.476 0.312 0.796 0.812 0.881 1.374 0.014 0.094 0.025 0.188 0.034 0.282 0.043-0.039 0.085-0.080 0.127-0.122 0.377-0.376 0.886-0.583 1.411-0.583 0.084 0 0.167 0.005 0.251 0.016 0.303 0.038 0.611 0.058 0.918 0.058 0.792 0 1.556-0.124 2.272-0.369 0.671-0.23 1.267-0.554 1.774-0.963 0.938-0.759 1.455-1.731 1.455-2.737s-0.517-1.978-1.455-2.737c-0.506-0.41-1.103-0.734-1.774-0.963-0.716-0.245-1.48-0.369-2.272-0.369zM7.5 0v0c4.142 0 7.5 2.717 7.5 6.069s-3.358 6.069-7.5 6.069c-0.398 0-0.788-0.025-1.169-0.074-1.611 1.605-3.471 1.892-5.331 1.935v-0.393c1.004-0.49 1.813-1.382 1.813-2.402 0-0.142-0.011-0.282-0.032-0.419-1.696-1.113-2.781-2.812-2.781-4.717 0-3.352 3.358-6.069 7.5-6.069zM15.563 13.604c0 0.874 0.567 1.639 1.438 2.059v0.337c-1.611-0.036-3.090-0.283-4.487-1.658-0.33 0.041-0.669 0.063-1.013 0.063-1.492 0-2.866-0.402-3.963-1.079 2.261-0.008 4.395-0.732 6.013-2.042 0.816-0.66 1.459-1.435 1.913-2.302 0.481-0.92 0.724-1.9 0.724-2.913 0-0.163-0.007-0.326-0.020-0.487 1.134 0.936 1.832 2.213 1.832 3.62 0 1.633-0.94 3.089-2.41 4.043-0.018 0.117-0.027 0.237-0.027 0.359z"
        /> < title > { title } < / title > < / svg >
    }
}
