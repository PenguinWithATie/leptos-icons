#[cfg(feature = "HiLgSolidDocumentMagnifyingGlass")]
use leptos::*;
#[cfg(feature = "HiLgSolidDocumentMagnifyingGlass")]
///This icon requires the feature `HiLgSolidDocumentMagnifyingGlass` to be enabled.
#[component]
pub fn DocumentMagnifyingGlass(
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
        "M11.625 16.5C12.6605 16.5 13.5 15.6605 13.5 14.625C13.5 13.5895 12.6605 12.75 11.625 12.75C10.5895 12.75 9.75 13.5895 9.75 14.625C9.75 15.6605 10.5895 16.5 11.625 16.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5.625 1.5H9C11.0711 1.5 12.75 3.17893 12.75 5.25V7.125C12.75 8.16053 13.5895 9 14.625 9H16.5C18.5711 9 20.25 10.6789 20.25 12.75V20.625C20.25 21.6605 19.4105 22.5 18.375 22.5H5.625C4.58947 22.5 3.75 21.6605 3.75 20.625V3.375C3.75 2.33947 4.58947 1.5 5.625 1.5ZM11.625 18C12.2854 18 12.9016 17.8103 13.4219 17.4824L14.4698 18.5303C14.7627 18.8232 15.2376 18.8232 15.5305 18.5303C15.8234 18.2374 15.8234 17.7626 15.5305 17.4697L14.4825 16.4217C14.8103 15.9014 15 15.2854 15 14.625C15 12.761 13.489 11.25 11.625 11.25C9.76104 11.25 8.25 12.761 8.25 14.625C8.25 16.489 9.76104 18 11.625 18Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.25 5.25C14.25 3.93695 13.768 2.73648 12.9712 1.8159C16.3701 2.70377 19.0462 5.37988 19.9341 8.77881C19.0135 7.98204 17.8131 7.5 16.5 7.5H14.625C14.4179 7.5 14.25 7.33211 14.25 7.125V5.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
