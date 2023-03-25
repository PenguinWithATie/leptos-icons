#[cfg(feature = "IoExtensionPuzzleOutline")]
use leptos::*;
#[cfg(feature = "IoExtensionPuzzleOutline")]
///This icon requires the feature `IoExtensionPuzzleOutline` to be enabled.
#[component]
pub fn ExtensionPuzzleOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" d =
        "M413.66,246.1H386a2,2,0,0,1-2-2V166.86A38.86,38.86,0,0,0,345.14,128H267.9a2,2,0,0,1-2-2V98.34c0-27.14-21.5-49.86-48.64-50.33a49.53,49.53,0,0,0-50.4,49.51V126a2,2,0,0,1-2,2H87.62A39.74,39.74,0,0,0,48,167.62V238a2,2,0,0,0,2,2H76.91c29.37,0,53.68,25.48,54.09,54.85.42,29.87-23.51,57.15-53.29,57.15H50a2,2,0,0,0-2,2v70.38A39.74,39.74,0,0,0,87.62,464H158a2,2,0,0,0,2-2V441.07c0-30.28,24.75-56.35,55-57.06,30.1-.7,57,20.31,57,50.28V462a2,2,0,0,0,2,2h71.14A38.86,38.86,0,0,0,384,425.14v-78a2,2,0,0,1,2-2h28.48c27.63,0,49.52-22.67,49.52-50.4S440.8,246.1,413.66,246.1Z"
        /> < title > { title } < / title > < / svg >
    }
}
