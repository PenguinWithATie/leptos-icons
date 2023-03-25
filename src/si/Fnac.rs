#[cfg(feature = "SiFnac")]
use leptos::*;
#[cfg(feature = "SiFnac")]
///This icon requires the feature `SiFnac` to be enabled.
#[component]
pub fn Fnac(
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
        "M20.874 7.2s-1.622-.106-1.684 1.369v.04c.062 1.476 1.684 1.39 1.684 1.39.737 0 1.137-.23 1.326-.652h.842l.232-1.495H22.18c-.19-.42-.569-.673-1.305-.652zm-4.59 1.516c-.147.19-.862.19-.862.19-.717.042-1.79.02-1.769.652.02.673.884.59 1.305.569.464-.021.863-.148 1.18-.548.294-.337.168-.842.146-.863.022-.021 0 0 0 0zM3.295 0l-1.01 6.358h.442c-.02-.19-.02-.97.547-1.474 0 0 .632-.632 2.485-.485 0 0 1.894.148 1.894 1.516H6.347s-.042-.757-1.22-.715c0 0-1.2-.021-1.096 1.137h3.621v.59c.106-.17.38-.443 1.074-.632 0 0 2.968-.654 3.284 1.474v2.989h-1.304V8.316C10.516 6.99 9.02 7.221 9.02 7.221c-1.011.084-1.306.673-1.369 1.095V10.8H6.347V7.263H4.052V10.8H2.747V7.263h-.59L.01 20.716 20.726 24l2.148-13.622c-.442.316-1.179.548-2.358.485-.484-.021-.863-.085-1.179-.21-.59-.21-1.536-.822-1.536-2.001v-.147c-.022-1.16.947-1.769 1.536-2 .316-.126.695-.17 1.18-.21 1.768-.106 2.525.483 2.841.989l.633-4.043zM16.37 10.799l-.043-.505v-.02c-.526.652-1.789.609-1.789.609-2.358.043-2.316-1.241-2.316-1.241-.19-1.348 1.537-1.327 2.968-1.41 1.432-.085 1.095-.653 1.095-.653-.063-.464-1.094-.506-1.094-.506-1.432-.105-1.453.757-1.453.757H12.39s0-1.136 1.411-1.452c.02 0 .842-.252 2.295 0 0 0 1.62.21 1.516 1.768l.042 2.674H16.37z"
        /> < title > { title } < / title > < / svg >
    }
}
