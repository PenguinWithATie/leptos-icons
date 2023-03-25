#[cfg(feature = "HiLgSolidCursorArrowRays")]
use leptos::*;
#[cfg(feature = "HiLgSolidCursorArrowRays")]
///This icon requires the feature `HiLgSolidCursorArrowRays` to be enabled.
#[component]
pub fn CursorArrowRays(
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
        "M12 1.5C12.4142 1.5 12.75 1.83579 12.75 2.25V4.5C12.75 4.91421 12.4142 5.25 12 5.25C11.5858 5.25 11.25 4.91421 11.25 4.5V2.25C11.25 1.83579 11.5858 1.5 12 1.5ZM5.63604 4.13604C5.92893 3.84315 6.40381 3.84315 6.6967 4.13604L8.28769 5.72703C8.58058 6.01992 8.58058 6.4948 8.28769 6.78769C7.9948 7.08058 7.51992 7.08058 7.22703 6.78769L5.63604 5.1967C5.34315 4.90381 5.34315 4.42893 5.63604 4.13604ZM18.364 4.13604C18.6569 4.42893 18.6569 4.90381 18.364 5.1967L16.773 6.78769C16.4801 7.08058 16.0052 7.08058 15.7123 6.78769C15.4194 6.4948 15.4194 6.01992 15.7123 5.72703L17.3033 4.13604C17.5962 3.84315 18.0711 3.84315 18.364 4.13604ZM11.5484 8.63179C11.8602 8.54824 12.1905 8.67359 12.3684 8.94299L17.5955 16.8599C17.7627 17.113 17.7609 17.4419 17.591 17.6932C17.421 17.9445 17.1165 18.0687 16.8193 18.0079L14.722 17.5787L15.7668 21.4777C15.874 21.8778 15.6365 22.289 15.2364 22.3963C14.8363 22.5035 14.4251 22.266 14.3179 21.8659L13.2732 17.967L11.6717 19.3872C11.4447 19.5884 11.1189 19.6332 10.8461 19.5005C10.5733 19.3678 10.4073 19.0839 10.4254 18.7811L10.9939 9.3113C11.0132 8.98905 11.2366 8.71534 11.5484 8.63179ZM3 10.5C3 10.0858 3.33579 9.75 3.75 9.75H6C6.41421 9.75 6.75 10.0858 6.75 10.5C6.75 10.9142 6.41421 11.25 6 11.25H3.75C3.33579 11.25 3 10.9142 3 10.5ZM17.25 10.5C17.25 10.0858 17.5858 9.75 18 9.75H20.25C20.6642 9.75 21 10.0858 21 10.5C21 10.9142 20.6642 11.25 20.25 11.25H18C17.5858 11.25 17.25 10.9142 17.25 10.5ZM8.28769 14.2123C8.58058 14.5052 8.58058 14.9801 8.28769 15.273L6.6967 16.864C6.40381 17.1569 5.92893 17.1569 5.63604 16.864C5.34315 16.5711 5.34315 16.0962 5.63604 15.8033L7.22703 14.2123C7.51992 13.9194 7.9948 13.9194 8.28769 14.2123Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
