#[cfg(feature = "HiMdSolidTicket")]
use leptos::*;
#[cfg(feature = "HiMdSolidTicket")]
///This icon requires the feature `HiMdSolidTicket` to be enabled.
#[component]
pub fn Ticket(
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
        "M13 3V4.27083C13 4.68505 13.3358 5.02083 13.75 5.02083C14.1642 5.02083 14.5 4.68505 14.5 4.27083V3H16.75C17.9926 3 19 4.00736 19 5.25V7.87803C19 8.19589 18.7996 8.47923 18.4999 8.58516C17.9163 8.79143 17.5 9.34806 17.5 10C17.5 10.6519 17.9163 11.2086 18.4999 11.4148C18.7996 11.5208 19 11.8041 19 12.122V14.75C19 15.9926 17.9926 17 16.75 17H14.5V15.7292C14.5 15.315 14.1642 14.9792 13.75 14.9792C13.3358 14.9792 13 15.315 13 15.7292V17H3.25C2.00736 17 1 15.9926 1 14.75V12.122C1 11.8041 1.20037 11.5208 1.50007 11.4148C2.08367 11.2086 2.5 10.6519 2.5 10C2.5 9.34806 2.08367 8.79143 1.50007 8.58516C1.20037 8.47923 1 8.19589 1 7.87803V5.25C1 4.00736 2.00736 3 3.25 3H13ZM14.5 7.39583C14.5 6.98162 14.1642 6.64583 13.75 6.64583C13.3358 6.64583 13 6.98162 13 7.39583V8.4375C13 8.85171 13.3358 9.1875 13.75 9.1875C14.1642 9.1875 14.5 8.85171 14.5 8.4375V7.39583ZM14.5 11.5625C14.5 11.1483 14.1642 10.8125 13.75 10.8125C13.3358 10.8125 13 11.1483 13 11.5625V12.6042C13 13.0184 13.3358 13.3542 13.75 13.3542C14.1642 13.3542 14.5 13.0184 14.5 12.6042V11.5625ZM6 10.75C6 10.3358 6.33579 10 6.75 10H10.25C10.6642 10 11 10.3358 11 10.75C11 11.1642 10.6642 11.5 10.25 11.5H6.75C6.33579 11.5 6 11.1642 6 10.75ZM6 13.25C6 12.8358 6.33579 12.5 6.75 12.5H8.25C8.66421 12.5 9 12.8358 9 13.25C9 13.6642 8.66421 14 8.25 14H6.75C6.33579 14 6 13.6642 6 13.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
