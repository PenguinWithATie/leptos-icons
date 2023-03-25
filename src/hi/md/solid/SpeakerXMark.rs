#[cfg(feature = "HiMdSolidSpeakerXMark")]
use leptos::*;
#[cfg(feature = "HiMdSolidSpeakerXMark")]
///This icon requires the feature `HiMdSolidSpeakerXMark` to be enabled.
#[component]
pub fn SpeakerXMark(
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
        "M9.54747 3.06153C9.82215 3.18021 10 3.4508 10 3.75002V16.25C10 16.5492 9.82215 16.8198 9.54747 16.9385C9.2728 17.0572 8.95387 17.0012 8.73598 16.7962L4.70257 13H3.16724C2.85725 13 2.5792 12.8093 2.46756 12.5201C2.16534 11.7372 2 10.887 2 10C2 9.11302 2.16534 8.26287 2.46756 7.47993C2.5792 7.19074 2.85725 7.00002 3.16724 7.00002H4.70257L8.73598 3.20387C8.95387 2.99879 9.2728 2.94286 9.54747 3.06153Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.2803 7.2197C12.9874 6.92681 12.5126 6.92681 12.2197 7.2197C11.9268 7.51259 11.9268 7.98747 12.2197 8.28036L13.9393 10L12.2197 11.7197C11.9268 12.0126 11.9268 12.4875 12.2197 12.7804C12.5126 13.0733 12.9874 13.0733 13.2803 12.7804L15 11.0607L16.7197 12.7804C17.0126 13.0733 17.4874 13.0733 17.7803 12.7804C18.0732 12.4875 18.0732 12.0126 17.7803 11.7197L16.0607 10L17.7803 8.28036C18.0732 7.98747 18.0732 7.51259 17.7803 7.2197C17.4874 6.92681 17.0126 6.92681 16.7197 7.2197L15 8.93937L13.2803 7.2197Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
