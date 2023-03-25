#[cfg(feature = "HiLgSolidHomeModern")]
use leptos::*;
#[cfg(feature = "HiLgSolidHomeModern")]
///This icon requires the feature `HiLgSolidHomeModern` to be enabled.
#[component]
pub fn HomeModern(
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
        "M19.0061 3.70505C19.3954 3.56349 19.5962 3.13317 19.4547 2.7439C19.3131 2.35462 18.8828 2.1538 18.4935 2.29536L6 6.83846V3.00021C6 2.58599 5.66421 2.25021 5.25 2.25021H3.75C3.33579 2.25021 3 2.58599 3 3.00021V7.92937L1.99353 8.29536C1.60426 8.43691 1.40344 8.86724 1.54499 9.25651C1.68655 9.64579 2.11687 9.8466 2.50615 9.70505L19.0061 3.70505Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3.01896 11.1147L18 5.66708V9.08858L22.0061 10.5454C22.3954 10.6869 22.5962 11.1172 22.4547 11.5065C22.3131 11.8958 21.8828 12.0966 21.4935 11.955L20.9998 11.7755V20.2502H21.75C22.1642 20.2502 22.5 20.586 22.5 21.0002C22.5 21.4144 22.1642 21.7502 21.75 21.7502H2.25C1.83579 21.7502 1.5 21.4144 1.5 21.0002C1.5 20.586 1.83579 20.2502 2.25 20.2502H3V11.1215L3.01896 11.1147ZM18 20.2502V10.6847L19.4998 11.2301V20.2502H18ZM9 14.2502C8.58579 14.2502 8.25 14.586 8.25 15.0002V19.5002C8.25 19.9144 8.58579 20.2502 9 20.2502H12C12.4142 20.2502 12.75 19.9144 12.75 19.5002V15.0002C12.75 14.586 12.4142 14.2502 12 14.2502H9Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
