#[cfg(feature = "SiLumen")]
use leptos::*;
#[cfg(feature = "SiLumen")]
///This icon requires the feature `SiLumen` to be enabled.
#[component]
pub fn Lumen(
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
        "M11.649 0a.75.75 0 00-.342.072l-4.878 2.23a.75.751 0 10.624 1.366l4.878-2.23A.75.75 0 0011.649 0zm5.624.354a.75.75 0 00-.341.074L6.425 5.306a.75.75 0 00.632 1.362L17.563 1.79a.75.75 0 00-.29-1.436zm0 3.002a.75.75 0 00-.341.074L6.425 8.31a.75.75 0 00.632 1.362l10.506-4.88a.75.75 0 00-.29-1.436zm0 3.002a.75.75 0 00-.341.074L6.425 11.311a.75.75 0 00.632 1.361l10.506-4.878a.75.75 0 00-.29-1.436zm.009 3.003a.75.75 0 00-.342.07l-3.753 1.688a.75.75 0 00-.442.685v3.518a.75.75 0 00.001.047h-1.503a.75.75 0 000-.047v-2.58a.75.75 0 00-.761-.761.75.75 0 00-.74.761v2.58a.75.75 0 00.002.047h-.94a.461.461 0 00-.47.555l.19 1.14a.687.687 0 00.656.557h2.28l-2.537.476a.375.375 0 10.139.737l6.003-1.126a.375.375 0 00.307-.41.625.625 0 00.092-.232l.19-1.142a.461.461 0 00-.47-.555h-.94a.75.75 0 00.002-.047V12.29l3.31-1.49a.75.75 0 00-.274-1.438zm-2.292 9.385a.375.375 0 00-.063.007l-6.004 1.126a.375.375 0 10.139.737l6.003-1.125a.375.375 0 00-.075-.745zm0 1.876a.375.375 0 00-.063.008l-6.004 1.125a.375.375 0 10.139.737l6.003-1.125a.375.375 0 00-.075-.745zm-.743 1.876a.375.375 0 00-.064.006l-4.471.751a.375.375 0 10.124.74l4.472-.75a.375.375 0 00-.061-.747z"
        /> < title > { title } < / title > < / svg >
    }
}
