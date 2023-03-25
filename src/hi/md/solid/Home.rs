#[cfg(feature = "HiMdSolidHome")]
use leptos::*;
#[cfg(feature = "HiMdSolidHome")]
///This icon requires the feature `HiMdSolidHome` to be enabled.
#[component]
pub fn Home(
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
        "M9.29292 2.29289C9.68345 1.90237 10.3166 1.90237 10.7071 2.29289L17.7071 9.29289C17.9931 9.57889 18.0787 10.009 17.9239 10.3827C17.7691 10.7564 17.4045 11 17 11H16V17C16 17.5523 15.5523 18 15 18H13C12.4477 18 12 17.5523 12 17V14C12 13.4477 11.5523 13 11 13H9.00003C8.44774 13 8.00003 13.4477 8.00003 14V17C8.00003 17.5523 7.55231 18 7.00003 18H5.00003C4.44774 18 4.00003 17.5523 4.00003 17V11H3.00003C2.59557 11 2.23093 10.7564 2.07615 10.3827C1.92137 10.009 2.00692 9.57889 2.29292 9.29289L9.29292 2.29289Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
