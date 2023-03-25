#[cfg(feature = "TiThumbsDown")]
use leptos::*;
#[cfg(feature = "TiThumbsDown")]
///This icon requires the feature `TiThumbsDown` to be enabled.
#[component]
pub fn ThumbsDown(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 5c-.755 0-1.438.289-1.965.751l-.188-.192c-.96-.737-3.665-1.559-5.847-1.559-1.879 0-2.607.293-3.252.552l-.316.124c-.834.305-1.578 1.229-1.738 2.2l-.664 5.972c-.174 1.039.441 2.127 1.4 2.478.394.144 2.512.405 3.883.56-.215 1.256-.312 2.405-.312 3.616 0 1.379 1.121 2.5 2.5 2.5s2.5-1.121 2.5-2.5c0-1.875.667-2.737 1.616-3.699.548.724 1.408 1.199 2.384 1.199 1.653 0 2.999-1.347 2.999-3v-6c-.001-1.656-1.346-3.002-3-3.002zm-6 14.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5c0-1.805.256-3.241.479-4.293l.297-1.398-1.321.188c-.605-.05-3.934-.447-4.335-.552-.058-.028-.132-.18-.108-.321l.663-5.976c.037-.223.291-.539.443-.594l.377-.146c.544-.219 1.015-.408 2.506-.408 1.914 0 4.118.753 4.633 1.146.156.12.366.564.366.854v4.977c-.001.026-.04.649-.707 1.316-.913.913-2.293 2.293-2.293 5.207zm7-5.5c0 .552-.448 1-1 1s-1-.448-1-1v-6c0-.552.448-1 1-1s1 .448 1 1v6z"
        /> < title > { title } < / title > < / svg >
    }
}
