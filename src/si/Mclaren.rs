#[cfg(feature = "SiMclaren")]
use leptos::*;
#[cfg(feature = "SiMclaren")]
///This icon requires the feature `SiMclaren` to be enabled.
#[component]
pub fn Mclaren(
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
        "M19.062 11.713c3.305-2.139 7.748-2.208 2.564 1.248l.082-.11c1.467-2.016-1.522-1.563-2.578-1.166l-.068.028zM6.967 13.236h1.399v.549H6.747c-.686 0-.987-.206-.987-.754v-.123c0-.466.274-.768.96-.768h1.646v.549H6.967a.248.248 0 0 0-.247.247v.069a.246.246 0 0 0 .247.231zM9.6 11.864v1.371h.823v.549h-1.92v-1.92H9.6zm-5.198.247c.191-.154.427-.241.672-.247h.549v1.92H4.525v-.96l-1.056.96H2.468v-.96l-1.221.96H0l2.18-1.646c.206-.151.343-.274.699-.274h.686v.96l.837-.713zm9.312.206a.316.316 0 0 1 .343-.316h1.303v.549h-.686v1.234h-.96v-1.467zm6.431-.316c.823 0 1.111.178 1.111.782v1.001h-.96v-.686a.411.411 0 0 0-.411-.411h-.411v1.097h-.96v-1.783h1.631zm-7.487 0c.631 0 .919.261.919.699v.411c0 .507-.288.672-.987.672h-1.083c-.398 0-.686-.041-.837-.178a.495.495 0 0 1-.11-.315v-.069c0-.274.165-.535.686-.535h1.234c0-.123.014-.137-.137-.137h-1.646V12h1.961zm-.179 1.166v-.069h-.754a.07.07 0 0 0 0 .138h.686a.068.068 0 0 0 .068-.069zm5.02-1.166c.727 0 .878.219.878.521v.069c0 .329-.261.507-.686.507h-1.234c0 .123.123.137.274.137h1.508v.549H16.36c-.59 0-.864-.247-.864-.699v-.315c0-.521.288-.768.946-.768h1.057zm-.151.686a.07.07 0 0 0 0-.138h-.823a.07.07 0 0 0-.069.069v.069h.892z"
        /> < title > { title } < / title > < / svg >
    }
}
