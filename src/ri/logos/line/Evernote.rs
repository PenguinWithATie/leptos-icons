#[cfg(feature = "RiLogosLineEvernote")]
use leptos::*;
#[cfg(feature = "RiLogosLineEvernote")]
///This icon requires the feature `RiLogosLineEvernote` to be enabled.
#[component]
pub fn Evernote(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M10.5 8.5a1 1 0 0 1-1 1H6.001c-.336 0-.501.261-.501.532 0 1.32.254 2.372.664 3.193.216.433.399.67.523.79.735.76 1.886 1.16 3.092 1.089.095-.006.199-.064.332-.208a1.51 1.51 0 0 0 .214-.293 2 2 0 0 1 2.531-1.073c.693.258 1.277.434 1.813.56.196.046.375.083.586.122-.077-.014.402.074.518.098.34.07.598.146.883.29a5.087 5.087 0 0 1 1.775 1.475c.045-.591.077-1.268.087-2.026a34.182 34.182 0 0 0-.559-6.673c-.074-.398-.236-.562-.663-.718a3.847 3.847 0 0 0-.587-.155c-.147-.028-.65-.11-.693-.118a1273 1273 0 0 1-2.34-.409l-.528-.092a2 2 0 0 1-1.524-1.26 11.467 11.467 0 0 0-.034-.088 5.595 5.595 0 0 0-.702-.036c-.271 0-.388.124-.388.463V8.5zm6.23 11.639c.352-.356.56-.829.587-1.327.054-1.036-.824-2.48-2.317-2.634-.617-.063-1.586-.306-2.842-.774 0 0-.7 1.603-2.26 1.696-1.665.1-3.43-.433-4.65-1.696 0 0-1.748-1.64-1.748-5.372 0-.814.29-1.422.648-1.904.96-1.292 2.505-2.78 4.133-4.304C9 3.15 9.701 2.5 10.888 2.5c2.04 0 2.32.664 2.605 1.414l2.854.499c.907.166 3.15.316 3.578 2.594 1.006 5.42.458 9.87.347 10.675-.71 5.121-4.772 4.871-4.931 4.871-2.059 0-3.178-1.373-3.183-2.677a2.494 2.494 0 0 1 1.038-2.034 2.586 2.586 0 0 1 1.527-.478c.305 0 .687.318.687.753 0 .37-.255.575-.382.645-.223.124-1.122.174-1.122.865 0 .317.35 1.114 1.386 1.114.588 0 1.094-.256 1.437-.602zm-1.796-9.51c.166-.415.627-.632 1.172-.582.544.067.956.4 1.006.848 0 .083.017.183-.017.233-.032.05-.066.067-.1.067-.213.033-.543 0-1.021-.05-.48-.05-.808-.1-1.006-.2-.033-.017-.066-.033-.083-.083s.016-.15.05-.233z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
