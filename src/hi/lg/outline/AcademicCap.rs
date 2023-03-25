#[cfg(feature = "HiLgOutlineAcademicCap")]
use leptos::*;
#[cfg(feature = "HiLgOutlineAcademicCap")]
///This icon requires the feature `HiLgOutlineAcademicCap` to be enabled.
#[component]
pub fn AcademicCap(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.25933 10.1468C3.98688 12.2308 3.82139 14.3485 3.76853 16.4941C6.66451 17.7032 9.41893 19.1836 12 20.9037C14.5811 19.1836 17.3355 17.7032 20.2315 16.4941C20.1786 14.3485 20.0131 12.2308 19.7407 10.1468M4.25933 10.1468C3.38362 9.85242 2.49729 9.5812 1.60107 9.33382C4.84646 7.05899 8.32741 5.09732 12 3.49268C15.6727 5.09732 19.1536 7.059 22.399 9.33383C21.5028 9.58122 20.6164 9.85245 19.7407 10.1468M4.25933 10.1468C6.94656 11.05 9.5338 12.171 12.0001 13.4888C14.4663 12.171 17.0535 11.0501 19.7407 10.1468M6.75 15.0001C7.16421 15.0001 7.5 14.6643 7.5 14.2501C7.5 13.8359 7.16421 13.5001 6.75 13.5001C6.33579 13.5001 6 13.8359 6 14.2501C6 14.6643 6.33579 15.0001 6.75 15.0001ZM6.75 15.0001V11.3246C8.44147 10.2736 10.1936 9.31107 12 8.44342M4.99264 19.9928C6.16421 18.8212 6.75 17.2857 6.75 15.7501V14.2501"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
