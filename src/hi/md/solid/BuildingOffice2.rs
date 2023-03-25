#[cfg(feature = "HiMdSolidBuildingOffice2")]
use leptos::*;
#[cfg(feature = "HiMdSolidBuildingOffice2")]
///This icon requires the feature `HiMdSolidBuildingOffice2` to be enabled.
#[component]
pub fn BuildingOffice2(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1 2.75C1 2.33579 1.33579 2 1.75 2H12.25C12.6642 2 13 2.33579 13 2.75C13 3.16421 12.6642 3.5 12.25 3.5H12V17.25C12 17.6642 11.6642 18 11.25 18H9.75C9.33579 18 9 17.6642 9 17.25V14.75C9 14.3358 8.66421 14 8.25 14H5.75C5.33579 14 5 14.3358 5 14.75V17.25C5 17.6642 4.66421 18 4.25 18H1.75C1.33579 18 1 17.6642 1 17.25C1 16.8358 1.33579 16.5 1.75 16.5H2V3.5H1.75C1.33579 3.5 1 3.16421 1 2.75ZM4 5.5C4 5.22386 4.22386 5 4.5 5H5.5C5.77614 5 6 5.22386 6 5.5V6.5C6 6.77614 5.77614 7 5.5 7H4.5C4.22386 7 4 6.77614 4 6.5V5.5ZM4.5 9C4.22386 9 4 9.22386 4 9.5V10.5C4 10.7761 4.22386 11 4.5 11H5.5C5.77614 11 6 10.7761 6 10.5V9.5C6 9.22386 5.77614 9 5.5 9H4.5ZM8 5.5C8 5.22386 8.22386 5 8.5 5H9.5C9.77614 5 10 5.22386 10 5.5V6.5C10 6.77614 9.77614 7 9.5 7H8.5C8.22386 7 8 6.77614 8 6.5V5.5ZM8.5 9C8.22386 9 8 9.22386 8 9.5V10.5C8 10.7761 8.22386 11 8.5 11H9.5C9.77614 11 10 10.7761 10 10.5V9.5C10 9.22386 9.77614 9 9.5 9H8.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M14.25 6C13.8358 6 13.5 6.33579 13.5 6.75V17C13.5 17.5523 13.9477 18 14.5 18H18.25C18.6642 18 19 17.6642 19 17.25C19 16.8358 18.6642 16.5 18.25 16.5H18V7.5H18.25C18.6642 7.5 19 7.16421 19 6.75C19 6.33579 18.6642 6 18.25 6H14.25ZM14.75 9.5C14.75 9.22386 14.9739 9 15.25 9H16.25C16.5261 9 16.75 9.22386 16.75 9.5V10.5C16.75 10.7761 16.5261 11 16.25 11H15.25C14.9739 11 14.75 10.7761 14.75 10.5V9.5ZM15.25 13C14.9739 13 14.75 13.2239 14.75 13.5V14.5C14.75 14.7761 14.9739 15 15.25 15H16.25C16.5261 15 16.75 14.7761 16.75 14.5V13.5C16.75 13.2239 16.5261 13 16.25 13H15.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
