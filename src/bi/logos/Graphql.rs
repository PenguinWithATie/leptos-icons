#[cfg(feature = "BiLogosGraphql")]
use leptos::*;
#[cfg(feature = "BiLogosGraphql")]
///This icon requires the feature `BiLogosGraphql` to be enabled.
#[component]
pub fn Graphql(
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
        "M20.1 14.56a2.07 2.07 0 0 0-.47-.18V9.62a1.64 1.64 0 0 0 .48-.18 1.78 1.78 0 0 0-1.78-3.09 1.62 1.62 0 0 0-.41.32l-4.11-2.38a1.7 1.7 0 0 0 .07-.51 1.78 1.78 0 0 0-3.56 0 1.7 1.7 0 0 0 .07.51L6.28 6.66a1.58 1.58 0 0 0-.41-.31 1.78 1.78 0 0 0-1.78 3.09 1.64 1.64 0 0 0 .48.18v4.76a2.07 2.07 0 0 0-.47.18 1.78 1.78 0 1 0 1.78 3.09 1.72 1.72 0 0 0 .4-.31l4.11 2.37a1.7 1.7 0 0 0-.07.51 1.78 1.78 0 0 0 3.56 0 1.69 1.69 0 0 0-.09-.56l4.09-2.36a1.7 1.7 0 0 0 .44.35 1.78 1.78 0 1 0 1.78-3.09zM6.72 15.69a1.72 1.72 0 0 0-.19-.47 1.53 1.53 0 0 0-.31-.4l5.38-9.33a1.82 1.82 0 0 0 1 0l5.4 9.33a1.53 1.53 0 0 0-.31.4 1.72 1.72 0 0 0-.19.47zM17.5 7.4a1.81 1.81 0 0 0 .17 1.38 1.75 1.75 0 0 0 1.12.84v4.76h-.07l-5.39-9.31.05-.07zM10.82 5a.12.12 0 0 0 0 .05L5.48 14.4h-.07V9.62a1.75 1.75 0 0 0 1.12-.84A1.81 1.81 0 0 0 6.7 7.4zm2.6 14a1.78 1.78 0 0 0-1.32-.58 1.75 1.75 0 0 0-1.28.54L6.7 16.6v-.06h10.78v.11z"
        /> < title > { title } < / title > < / svg >
    }
}
