#[cfg(feature = "HiLgOutlineDocumentDuplicate")]
use leptos::*;
#[cfg(feature = "HiLgOutlineDocumentDuplicate")]
///This icon requires the feature `HiLgOutlineDocumentDuplicate` to be enabled.
#[component]
pub fn DocumentDuplicate(
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
        "M15.75 17.25V20.625C15.75 21.2463 15.2463 21.75 14.625 21.75H4.875C4.25368 21.75 3.75 21.2463 3.75 20.625V7.875C3.75 7.25368 4.25368 6.75 4.875 6.75H6.75C7.26107 6.75 7.76219 6.7926 8.25 6.87444M15.75 17.25H19.125C19.7463 17.25 20.25 16.7463 20.25 16.125V11.25C20.25 6.79051 17.0066 3.08855 12.75 2.37444C12.2622 2.2926 11.7611 2.25 11.25 2.25H9.375C8.75368 2.25 8.25 2.75368 8.25 3.375V6.87444M15.75 17.25H9.375C8.75368 17.25 8.25 16.7463 8.25 16.125V6.87444M20.25 13.5V11.625C20.25 9.76104 18.739 8.25 16.875 8.25H15.375C14.7537 8.25 14.25 7.74632 14.25 7.125V5.625C14.25 3.76104 12.739 2.25 10.875 2.25H9.75"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
