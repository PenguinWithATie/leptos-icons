#[cfg(feature = "SiPhotobucket")]
use leptos::*;
#[cfg(feature = "SiPhotobucket")]
///This icon requires the feature `SiPhotobucket` to be enabled.
#[component]
pub fn Photobucket(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.924 11.451c.059.629.023.85-.168 1.042-.288.288-.656.312-.969.064-.127-.1-.269-.52-.322-.951a4.842 4.842 0 0 0-1.09-2.492c-.57-.692-2.005-1.407-2.822-1.407-.652 0-1.199-.361-1.199-.791 0-.266.507-.79.764-.79.104 0 .57.071 1.035.156 2.543.47 4.532 2.625 4.771 5.169zM24 12.05c0 6.531-.008 6.731-.288 7.01-.27.27-.479.287-3.453.287h-3.165l-1.002.504c-.551.277-1.406.6-1.901.716-1.123.265-3.071.257-4.204-.016-.474-.115-1.304-.433-1.844-.707l-.981-.498H3.82c-3.057 0-3.364-.023-3.581-.263-.215-.236-.239-.938-.239-7.09 0-7.624-.054-7.23 1.006-7.329.411-.039.512-.112.553-.406.086-.625.274-.719 1.448-.719 1.213 0 1.448.121 1.448.747v.402h1.302c1.215 0 1.381-.037 2.479-.558 1.515-.718 2.274-.892 3.908-.892 1.644 0 2.409.175 3.885.891l1.155.56h3.184c3.127 0 3.188.006 3.408.319.195.278.224 1.19.224 7.042zM2.587 4.545c0 .079.194.144.431.144s.431-.065.431-.144c0-.079-.194-.144-.431-.144s-.431.065-.431.144zm2.874 13.294c0-.039-.21-.346-.467-.683-2.108-2.761-2.184-7.023-.18-10.022l.626-.936-1.802-.04c-1.204-.028-1.867.014-2.001.125-.164.136-.2 1.14-.2 5.618 0 3.484.054 5.553.149 5.731.135.253.317.278 2.012.278 1.025 0 1.863-.032 1.863-.071zm13.183-9.197c-.517-1.033-2.026-2.573-3.061-3.125-2.068-1.102-4.809-1.102-6.877 0-1.166.622-2.586 2.127-3.17 3.36a7.258 7.258 0 0 0 3.454 9.704c1.532.731 2.931.922 4.535.62 1.072-.202 2.65-.986 3.494-1.737 2.453-2.18 3.124-5.825 1.625-8.822zm3.741 3.294l-.038-5.739-1.781-.041-1.781-.04.237.328c2.419 3.354 2.581 7.219.437 10.416-.348.519-.633.967-.633.996 0 .108 3.24.044 3.415-.067.142-.089.173-1.357.144-5.853z"
        /> < title > { title } < / title > < / svg >
    }
}
