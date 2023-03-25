#[cfg(feature = "IoFastFood")]
use leptos::*;
#[cfg(feature = "IoFastFood")]
///This icon requires the feature `IoFastFood` to be enabled.
#[component]
pub fn FastFood(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,128h.09" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M479.55,96H388.49l8.92-35.66,38.32-13.05c8.15-2.77,13-11.43,10.65-19.71a16,16,0,0,0-20.54-10.73l-47,16a16,16,0,0,0-10.36,11.27L355.51,96H224.45c-8.61,0-16,6.62-16.43,15.23A16,16,0,0,0,224,128h2.75l1,8.66A8.3,8.3,0,0,0,236,144h0c39,0,73.66,10.9,100.12,31.52A121.9,121.9,0,0,1,371,218.07a123.4,123.4,0,0,1,10.12,29.51,7.83,7.83,0,0,0,3.29,4.88,72,72,0,0,1,26.38,86.43,7.92,7.92,0,0,0-.15,5.53A96,96,0,0,1,416,376c0,22.34-7.6,43.63-21.4,59.95a80.12,80.12,0,0,1-28.78,21.67,8,8,0,0,0-4.21,4.37,108.19,108.19,0,0,1-17.37,29.86l0,0a2.5,2.5,0,0,0,1.9,4.11h49.21a48.22,48.22,0,0,0,47.85-44.14L477.4,128H480a16,16,0,0,0,16-16.77C495.58,102.62,488.16,96,479.55,96Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M108.69,320a23.87,23.87,0,0,1,17,7l15.51,15.51a4,4,0,0,0,5.66,0L162.34,327a23.87,23.87,0,0,1,17-7H375.92a8,8,0,0,0,8.08-7.92V312a40.07,40.07,0,0,0-32-39.2c-.82-29.69-13-54.54-35.51-72C295.67,184.56,267.85,176,236,176H164c-68.22,0-114.43,38.77-116,96.8A40.07,40.07,0,0,0,16,312h0a8,8,0,0,0,8,8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M185.94,352a8,8,0,0,0-5.66,2.34l-22.14,22.15a20,20,0,0,1-28.28,0l-22.14-22.15a8,8,0,0,0-5.66-2.34H32.66A15.93,15.93,0,0,0,16.9,365.17,65.22,65.22,0,0,0,16,376c0,30.59,21.13,55.51,47.26,56,2.43,15.12,8.31,28.78,17.16,39.47C93.51,487.28,112.54,496,134,496H266c21.46,0,40.49-8.72,53.58-24.55,8.85-10.69,14.73-24.35,17.16-39.47,26.13-.47,47.26-25.39,47.26-56a65.22,65.22,0,0,0-.9-10.83A15.93,15.93,0,0,0,367.34,352Z"
        /> < title > { title } < / title > < / svg >
    }
}
