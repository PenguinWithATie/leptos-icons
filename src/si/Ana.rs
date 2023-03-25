#[cfg(feature = "SiAna")]
use leptos::*;
#[cfg(feature = "SiAna")]
///This icon requires the feature `SiAna` to be enabled.
#[component]
pub fn Ana(
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
        "M19.11 8.975l-3.454 6.05h3.432l3.455-6.05zm3.933 0l-3.455 6.05h.959L24 8.975zm-10.01.781H14.8l.403 5.27h-1.31l-.025-.58a.206.206 0 0 0-.202-.227h-1.867l.429-.757h1.21c.151 0 .328.026.328-.202l-.202-2.37c0-.15-.126-.226-.227-.075L11.193 15h-.882zm-9.983 0h1.74l.353 5.27h-1.31l-.026-.58a.226.226 0 0 0-.227-.227H1.563l.429-.757h1.386c.151 0 .328.026.328-.202l-.151-2.37c0-.15-.126-.226-.227-.075L.882 15H0zm3.278 0h1.79l1.16 4.084c.05.126.15.101.176 0l.756-4.084h.782l-.933 5.27H8.244l-1.135-4.034c-.025-.101-.151-.127-.176 0l-.706 4.033h-.832Z"
        /> < title > { title } < / title > < / svg >
    }
}
