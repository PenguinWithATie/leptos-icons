#[cfg(feature = "HiLgSolidCursorArrowRipple")]
use leptos::*;
#[cfg(feature = "HiLgSolidCursorArrowRipple")]
///This icon requires the feature `HiLgSolidCursorArrowRipple` to be enabled.
#[component]
pub fn CursorArrowRipple(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M17.3033 5.1967C14.3744 2.26777 9.62563 2.26777 6.6967 5.1967C3.76777 8.12563 3.76777 12.8744 6.6967 15.8033C6.98959 16.0962 6.98959 16.5711 6.6967 16.864C6.40381 17.1569 5.92893 17.1569 5.63604 16.864C2.12132 13.3492 2.12132 7.65076 5.63604 4.13604C9.15076 0.62132 14.8492 0.62132 18.364 4.13604C20.1211 5.89321 21 8.19775 21 10.4998C21 10.9141 20.6642 11.2498 20.25 11.2499C19.8358 11.2499 19.5 10.9141 19.5 10.4999C19.5 8.57933 18.7679 6.66128 17.3033 5.1967ZM15.182 7.31802C13.4246 5.56066 10.5754 5.56066 8.81802 7.31802C7.06066 9.07538 7.06066 11.9246 8.81802 13.682C9.11091 13.9749 9.11091 14.4497 8.81802 14.7426C8.52513 15.0355 8.05025 15.0355 7.75736 14.7426C5.41421 12.3995 5.41421 8.60051 7.75736 6.25736C10.1005 3.91421 13.8995 3.91421 16.2426 6.25736C17.414 7.42877 18 8.96558 18 10.4999C18 10.9141 17.6642 11.2499 17.25 11.2499C16.8358 11.2499 16.5 10.9142 16.5 10.4999C16.5 9.34715 16.0608 8.19683 15.182 7.31802ZM11.5484 8.63179C11.8602 8.54824 12.1905 8.67359 12.3684 8.94299L17.5955 16.8599C17.7627 17.113 17.7609 17.4419 17.591 17.6932C17.421 17.9445 17.1165 18.0687 16.8193 18.0079L14.722 17.5787L15.7668 21.4777C15.874 21.8778 15.6365 22.289 15.2364 22.3963C14.8363 22.5035 14.4251 22.266 14.3179 21.8659L13.2732 17.967L11.6717 19.3872C11.4447 19.5884 11.1189 19.6332 10.8461 19.5005C10.5733 19.3678 10.4073 19.0839 10.4254 18.7811L10.9939 9.3113C11.0132 8.98905 11.2366 8.71534 11.5484 8.63179Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
