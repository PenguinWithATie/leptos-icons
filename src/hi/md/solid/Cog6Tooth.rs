#[cfg(feature = "HiMdSolidCog6Tooth")]
use leptos::*;
#[cfg(feature = "HiMdSolidCog6Tooth")]
///This icon requires the feature `HiMdSolidCog6Tooth` to be enabled.
#[component]
pub fn Cog6Tooth(
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
        "M7.83922 1.80388C7.9327 1.33646 8.34312 1 8.8198 1H11.1802C11.6569 1 12.0673 1.33646 12.1608 1.80388L12.4913 3.45629C13.1956 3.72458 13.8454 4.10332 14.4196 4.57133L16.0179 4.03065C16.4694 3.8779 16.966 4.06509 17.2043 4.47791L18.3845 6.52207C18.6229 6.93489 18.5367 7.45855 18.1786 7.77322L16.9119 8.88645C16.9699 9.24909 17 9.62103 17 10C17 10.379 16.9699 10.7509 16.9119 11.1135L18.1786 12.2268C18.5367 12.5414 18.6229 13.0651 18.3845 13.4779L17.2043 15.5221C16.966 15.9349 16.4694 16.1221 16.0179 15.9693L14.4196 15.4287C13.8454 15.8967 13.1956 16.2754 12.4913 16.5437L12.1608 18.1961C12.0673 18.6635 11.6569 19 11.1802 19H8.8198C8.34312 19 7.9327 18.6635 7.83922 18.1961L7.50874 16.5437C6.80442 16.2754 6.1546 15.8967 5.58042 15.4287L3.98213 15.9694C3.53059 16.1221 3.034 15.9349 2.79566 15.5221L1.61546 13.4779C1.37712 13.0651 1.4633 12.5415 1.82136 12.2268L3.08808 11.1135C3.03011 10.7509 2.99999 10.379 2.99999 10C2.99999 9.62103 3.03011 9.2491 3.08808 8.88647L1.82136 7.77324C1.4633 7.45857 1.37712 6.93491 1.61546 6.52209L2.79566 4.47793C3.034 4.06511 3.53059 3.87791 3.98213 4.03066L5.58041 4.57134C6.15459 4.10332 6.80442 3.72459 7.50874 3.45629L7.83922 1.80388ZM9.99999 13C11.6568 13 13 11.6569 13 10C13 8.34315 11.6568 7 9.99999 7C8.34314 7 6.99999 8.34315 6.99999 10C6.99999 11.6569 8.34314 13 9.99999 13Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
