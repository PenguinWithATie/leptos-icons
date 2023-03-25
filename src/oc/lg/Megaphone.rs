#[cfg(feature = "OcLgMegaphone")]
use leptos::*;
#[cfg(feature = "OcLgMegaphone")]
///This icon requires the feature `OcLgMegaphone` to be enabled.
#[component]
pub fn Megaphone(
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
        "M22 1.75v14.5a.75.75 0 0 1-.399.662c-.384.204-.783-.035-1.139-.248l-.003-.002c-.09-.054-.177-.107-.261-.15a15.53 15.53 0 0 0-2-.849c-1.738-.607-4.321-1.223-7.703-1.251a.833.833 0 0 1 .005.088c0 2.279.494 4.279.906 5.547.368 1.131-.438 2.453-1.732 2.453H7.661c-.696 0-1.36-.42-1.6-1.129C5.684 20.255 5 17.811 5 14.75v-.457A5.5 5.5 0 0 1 6.5 3.5h3.75c3.505 0 6.175-.61 7.955-1.21a15.88 15.88 0 0 0 2.002-.82 9.21 9.21 0 0 0 .49-.262c.048-.028.095-.055.142-.085A.751.751 0 0 1 22 1.75ZM10.5 12.912c3.564.029 6.313.678 8.193 1.335.737.258 1.34.517 1.807.74V2.993c-.467.216-1.073.467-1.815.718-1.878.634-4.624 1.26-8.185 1.288ZM6.5 5a4 4 0 0 0 0 8H9V5Zm0 9.75c0 2.847.638 5.123.982 6.141.018.051.074.109.179.109h2.013c.087 0 .179-.043.249-.147a.396.396 0 0 0 .057-.343C9.537 19.148 9 16.986 9 14.5H6.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
