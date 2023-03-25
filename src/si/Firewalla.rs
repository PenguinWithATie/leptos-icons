#[cfg(feature = "SiFirewalla")]
use leptos::*;
#[cfg(feature = "SiFirewalla")]
///This icon requires the feature `SiFirewalla` to be enabled.
#[component]
pub fn Firewalla(
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
        "M12.156 0c-3.602 4.89.391 7.768 2.61 11.893-.751 1.527-1.745 3.08-2.733 4.836l-.072.025c-.849-.983-1.99-1.85-3.033-2.967 2.606-5.783-.809-9.812-.809-9.812a12.555 12.555 0 0 0-1.916 2.021c-2.296 3.06-2.027 5.897-2.027 5.897C4.176 19.07 12.125 24 12.125 24a21.738 21.738 0 0 0 2.139-1.594c5.864-4.974 5.564-10.513 5.564-10.513.122-4.308-1.622-5.905-4.82-9.014A83.864 83.864 0 0 1 12.156 0zm.281 17.37zm.397.687a4.298 4.298 0 0 1 .14.328 4.463 4.463 0 0 0-.14-.328zm.266.718a4.289 4.289 0 0 1 .14.791c.024.286.023.588-.006.91a5.23 5.23 0 0 0 .006-.91 4.513 4.513 0 0 0-.14-.79z"
        /> < title > { title } < / title > < / svg >
    }
}
