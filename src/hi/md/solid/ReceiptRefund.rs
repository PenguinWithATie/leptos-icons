#[cfg(feature = "HiMdSolidReceiptRefund")]
use leptos::*;
#[cfg(feature = "HiMdSolidReceiptRefund")]
///This icon requires the feature `HiMdSolidReceiptRefund` to be enabled.
#[component]
pub fn ReceiptRefund(
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
        "M4.93005 1.31046C6.5916 1.10551 8.28365 1 10 1C11.7163 1 13.4084 1.10551 15.07 1.31046C16.1942 1.44913 17 2.41374 17 3.51661V18.25C17 18.5078 16.8676 18.7475 16.6494 18.8848C16.4312 19.0221 16.1578 19.0377 15.9255 18.9261L13.125 17.5819L10.3245 18.9261C10.1194 19.0246 9.88061 19.0246 9.67545 18.9261L6.875 17.5819L4.07455 18.9261C3.84215 19.0377 3.56875 19.0221 3.35057 18.8848C3.13239 18.7475 3 18.5078 3 18.25V3.51661C3 2.41374 3.80579 1.44913 4.93005 1.31046ZM9.75172 6.30747C10.0596 6.03038 10.0846 5.55616 9.80747 5.24828C9.53038 4.94039 9.05616 4.91544 8.74828 5.19253L6.24828 7.44253C6.09024 7.58476 6 7.78738 6 8C6 8.21262 6.09024 8.41524 6.24828 8.55747L8.74828 10.8075C9.05616 11.0846 9.53038 11.0596 9.80747 10.7517C10.0846 10.4438 10.0596 9.96962 9.75172 9.69253L8.70447 8.75H10.625C11.6605 8.75 12.5 9.58947 12.5 10.625C12.5 11.6605 11.6605 12.5 10.625 12.5C10.2108 12.5 9.875 12.8358 9.875 13.25C9.875 13.6642 10.2108 14 10.625 14C12.489 14 14 12.489 14 10.625C14 8.76104 12.489 7.25 10.625 7.25H8.70447L9.75172 6.30747Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
