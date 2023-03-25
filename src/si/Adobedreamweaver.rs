#[cfg(feature = "SiAdobedreamweaver")]
use leptos::*;
#[cfg(feature = "SiAdobedreamweaver")]
///This icon requires the feature `SiAdobedreamweaver` to be enabled.
#[component]
pub fn Adobedreamweaver(
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
        "M6.69 8.4c-.43-.13-.87-.2-1.32-.19-.2 0-.37 0-.51.01-.14 0-.3.01-.47.02v6.67c.11 0 .21 0 .31.01.09.01.19.01.28.02.1.011.21.011.33.011.46.01.92-.07 1.36-.229.4-.141.75-.371 1.05-.681.3-.319.53-.7.67-1.11.16-.479.24-.99.24-1.5.01-.48-.07-.96-.23-1.41C8.12 9.24 7.49 8.64 6.69 8.4zM19.75.3H4.25C1.9.3 0 2.2 0 4.55v14.9c0 2.35 1.9 4.25 4.25 4.25h15.5c2.35 0 4.25-1.9 4.25-4.25V4.55C24 2.2 22.1.3 19.75.3zm-9.24 13.13c-.19.561-.48 1.08-.86 1.541-.35.42-.77.779-1.23 1.069-.45.28-.95.489-1.47.63-.5.13-1.02.2-1.54.2H4.28c-.4 0-.78 0-1.12-.011-.35-.01-.61-.01-.78-.02-.07 0-.1-.061-.1-.16V6.44c-.01-.06.04-.12.1-.13h.01c.15-.01.38-.02.67-.02.3-.01.64-.01 1.04-.02s.82-.01 1.27-.01c1.22 0 2.24.22 3.04.66.77.41 1.4 1.04 1.81 1.81.4.77.6 1.65.6 2.65.01.7-.09 1.39-.31 2.05zm9.42 3.24c-.01.04-.029.08-.06.109 0 .051-.04.07-.091.061H17.91c-.04.01-.09-.01-.12-.04-.03-.04-.05-.079-.06-.12-.19-.8-.351-1.52-.48-2.13-.13-.62-.24-1.14-.32-1.569-.08-.431-.15-.791-.209-1.09-.051-.3-.101-.55-.131-.76h-.01c-.1.44-.189.87-.28 1.28-.079.41-.18.83-.28 1.25-.1.42-.209.88-.34 1.38-.119.5-.26 1.05-.41 1.64-.02.11-.069.16-.16.16h-1.87c-.051.01-.1-.01-.141-.029-.029-.031-.05-.07-.07-.11L11.08 8.97c-.03-.09.01-.13.12-.13h1.89c.09 0 .141.03.15.1.199.88.369 1.64.5 2.28.13.64.24 1.18.31 1.629.07.45.14.82.19 1.101.05.28.09.521.119.7h.031c.02-.16.039-.311.069-.471.04-.189.09-.439.149-.75.061-.31.131-.67.221-1.09s.189-.9.311-1.46c.109-.55.27-1.18.459-1.89 0-.04.021-.09.041-.13.01-.02.049-.03.109-.03h1.96c.06 0 .09.04.101.11.17.73.31 1.37.43 1.92.109.55.21 1.04.3 1.47.08.42.149.79.19 1.09.039.311.09.561.129.77.031.17.061.34.07.511h.03c.05-.2.09-.44.13-.71.04-.271.09-.57.16-.91.061-.34.13-.71.21-1.12.069-.41.17-.86.28-1.37.109-.5.23-1.05.369-1.64.021-.09.061-.13.131-.13h1.75c.09 0 .119.05.1.14l-2.159 7.71z"
        /> < title > { title } < / title > < / svg >
    }
}
