#[cfg(feature = "FaSolidFileCsv")]
use leptos::*;
#[cfg(feature = "FaSolidFileCsv")]
///This icon requires the feature `FaSolidFileCsv` to be enabled.
#[component]
pub fn FileCsv(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V160H256c-17.7 0-32-14.3-32-32V0H64zM256 0V128H384L256 0zM80 224H96c22.1 0 40 17.9 40 40v8c0 8.8-7.2 16-16 16s-16-7.2-16-16v-8c0-4.4-3.6-8-8-8H80c-4.4 0-8 3.6-8 8v80c0 4.4 3.6 8 8 8H96c4.4 0 8-3.6 8-8v-8c0-8.8 7.2-16 16-16s16 7.2 16 16v8c0 22.1-17.9 40-40 40H80c-22.1 0-40-17.9-40-40V264c0-22.1 17.9-40 40-40zm72 46.4c0-25.6 20.8-46.4 46.4-46.4H216c8.8 0 16 7.2 16 16s-7.2 16-16 16H198.4c-7.9 0-14.4 6.4-14.4 14.4c0 5.2 2.8 9.9 7.2 12.5l25.4 14.5c14.4 8.3 23.4 23.6 23.4 40.3c0 25.6-20.8 46.4-46.4 46.4H168c-8.8 0-16-7.2-16-16s7.2-16 16-16h25.6c7.9 0 14.4-6.4 14.4-14.4c0-5.2-2.8-9.9-7.2-12.5l-25.4-14.5C160.9 302.4 152 287 152 270.4zM280 240v31.6c0 23 5.5 45.6 16 66c10.5-20.3 16-42.9 16-66V240c0-8.8 7.2-16 16-16s16 7.2 16 16v31.6c0 34.7-10.3 68.7-29.6 97.6l-5.1 7.7c-3 4.5-8 7.1-13.3 7.1s-10.3-2.7-13.3-7.1l-5.1-7.7c-19.3-28.9-29.6-62.9-29.6-97.6V240c0-8.8 7.2-16 16-16s16 7.2 16 16z"
        /> < title > { title } < / title > < / svg >
    }
}
