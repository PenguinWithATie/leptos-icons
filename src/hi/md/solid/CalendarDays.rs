#[cfg(feature = "HiMdSolidCalendarDays")]
use leptos::*;
#[cfg(feature = "HiMdSolidCalendarDays")]
///This icon requires the feature `HiMdSolidCalendarDays` to be enabled.
#[component]
pub fn CalendarDays(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.25 12C5.25 11.5858 5.58579 11.25 6 11.25H6.01C6.42421 11.25 6.76 11.5858 6.76 12V12.01C6.76 12.4242 6.42421 12.76 6.01 12.76H6C5.58579 12.76 5.25 12.4242 5.25 12.01V12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 13.25C5.58579 13.25 5.25 13.5858 5.25 14V14.01C5.25 14.4242 5.58579 14.76 6 14.76H6.01C6.42421 14.76 6.76 14.4242 6.76 14.01V14C6.76 13.5858 6.42421 13.25 6.01 13.25H6Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.25 12C7.25 11.5858 7.58579 11.25 8 11.25H8.01C8.42421 11.25 8.76 11.5858 8.76 12V12.01C8.76 12.4242 8.42421 12.76 8.01 12.76H8C7.58579 12.76 7.25 12.4242 7.25 12.01V12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13.25C7.58579 13.25 7.25 13.5858 7.25 14V14.01C7.25 14.4242 7.58579 14.76 8 14.76H8.01C8.42421 14.76 8.76 14.4242 8.76 14.01V14C8.76 13.5858 8.42421 13.25 8.01 13.25H8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.25 10C9.25 9.58579 9.58579 9.25 10 9.25H10.01C10.4242 9.25 10.76 9.58579 10.76 10V10.01C10.76 10.4242 10.4242 10.76 10.01 10.76H10C9.58579 10.76 9.25 10.4242 9.25 10.01V10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 11.25C9.58579 11.25 9.25 11.5858 9.25 12V12.01C9.25 12.4242 9.58579 12.76 10 12.76H10.01C10.4242 12.76 10.76 12.4242 10.76 12.01V12C10.76 11.5858 10.4242 11.25 10.01 11.25H10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.25 14C9.25 13.5858 9.58579 13.25 10 13.25H10.01C10.4242 13.25 10.76 13.5858 10.76 14V14.01C10.76 14.4242 10.4242 14.76 10.01 14.76H10C9.58579 14.76 9.25 14.4242 9.25 14.01V14Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9.25C11.5858 9.25 11.25 9.58579 11.25 10V10.01C11.25 10.4242 11.5858 10.76 12 10.76H12.01C12.4242 10.76 12.76 10.4242 12.76 10.01V10C12.76 9.58579 12.4242 9.25 12.01 9.25H12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.25 12C11.25 11.5858 11.5858 11.25 12 11.25H12.01C12.4242 11.25 12.76 11.5858 12.76 12V12.01C12.76 12.4242 12.4242 12.76 12.01 12.76H12C11.5858 12.76 11.25 12.4242 11.25 12.01V12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 13.25C11.5858 13.25 11.25 13.5858 11.25 14V14.01C11.25 14.4242 11.5858 14.76 12 14.76H12.01C12.4242 14.76 12.76 14.4242 12.76 14.01V14C12.76 13.5858 12.4242 13.25 12.01 13.25H12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.25 10C13.25 9.58579 13.5858 9.25 14 9.25H14.01C14.4242 9.25 14.76 9.58579 14.76 10V10.01C14.76 10.4242 14.4242 10.76 14.01 10.76H14C13.5858 10.76 13.25 10.4242 13.25 10.01V10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 11.25C13.5858 11.25 13.25 11.5858 13.25 12V12.01C13.25 12.4242 13.5858 12.76 14 12.76H14.01C14.4242 12.76 14.76 12.4242 14.76 12.01V12C14.76 11.5858 14.4242 11.25 14.01 11.25H14Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5.75 2C6.16421 2 6.5 2.33579 6.5 2.75V4H13.5V2.75C13.5 2.33579 13.8358 2 14.25 2C14.6642 2 15 2.33579 15 2.75V4H15.25C16.7688 4 18 5.23122 18 6.75V15.25C18 16.7688 16.7688 18 15.25 18H4.75C3.23122 18 2 16.7688 2 15.25V6.75C2 5.23122 3.23122 4 4.75 4H5V2.75C5 2.33579 5.33579 2 5.75 2ZM4.75 7.5C4.05964 7.5 3.5 8.05964 3.5 8.75V15.25C3.5 15.9404 4.05964 16.5 4.75 16.5H15.25C15.9404 16.5 16.5 15.9404 16.5 15.25V8.75C16.5 8.05964 15.9404 7.5 15.25 7.5H4.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
