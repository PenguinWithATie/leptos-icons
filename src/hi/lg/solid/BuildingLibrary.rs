#[cfg(feature = "HiLgSolidBuildingLibrary")]
use leptos::*;
#[cfg(feature = "HiLgSolidBuildingLibrary")]
///This icon requires the feature `HiLgSolidBuildingLibrary` to be enabled.
#[component]
pub fn BuildingLibrary(
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
        "M11.5841 2.37596C11.836 2.20801 12.1642 2.20801 12.4161 2.37596L21.4161 8.37596C21.7608 8.60573 21.8539 9.07138 21.6241 9.41602C21.3944 9.76067 20.9287 9.8538 20.5841 9.62404L12.0001 3.90139L3.4161 9.62404C3.07146 9.8538 2.60581 9.76067 2.37604 9.41602C2.14628 9.07138 2.23941 8.60573 2.58405 8.37596L11.5841 2.37596Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M20.25 10.3325V20.25H21C21.4142 20.25 21.75 20.5858 21.75 21C21.75 21.4142 21.4142 21.75 21 21.75H3C2.58579 21.75 2.25 21.4142 2.25 21C2.25 20.5858 2.58579 20.25 3 20.25H3.75V10.3325C3.75 9.96317 4.01888 9.64882 4.38374 9.59157C6.86578 9.20211 9.40954 9 12 9C14.5905 9 17.1342 9.20211 19.6163 9.59157C19.9811 9.64882 20.25 9.96317 20.25 10.3325ZM12.75 12.75C12.75 12.3358 12.4142 12 12 12C11.5858 12 11.25 12.3358 11.25 12.75V19.5C11.25 19.9142 11.5858 20.25 12 20.25C12.4142 20.25 12.75 19.9142 12.75 19.5V12.75ZM15.75 12C16.1642 12 16.5 12.3358 16.5 12.75V19.5C16.5 19.9142 16.1642 20.25 15.75 20.25C15.3358 20.25 15 19.9142 15 19.5V12.75C15 12.3358 15.3358 12 15.75 12ZM9 12.75C9 12.3358 8.66421 12 8.25 12C7.83579 12 7.5 12.3358 7.5 12.75V19.5C7.5 19.9142 7.83579 20.25 8.25 20.25C8.66421 20.25 9 19.9142 9 19.5V12.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7.875C12.6213 7.875 13.125 7.37132 13.125 6.75C13.125 6.12868 12.6213 5.625 12 5.625C11.3787 5.625 10.875 6.12868 10.875 6.75C10.875 7.37132 11.3787 7.875 12 7.875Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
