#[cfg(feature = "VsPaintcan")]
use leptos::*;
#[cfg(feature = "VsPaintcan")]
///This icon requires the feature `VsPaintcan` to be enabled.
#[component]
pub fn Paintcan(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.54 11.811l-1.14-3.12v-.06l-4.91-4.91v-1.24a1.66 1.66 0 0 0-.11-.58 1.48 1.48 0 0 0-.83-.8 1.42 1.42 0 0 0-.58-.1 1.47 1.47 0 0 0-1.48 1.48v3.26l-3.06 3a1.52 1.52 0 0 0 0 2.12l3.63 3.63c.14.141.307.253.49.33a1.53 1.53 0 0 0 1.14 0 1.51 1.51 0 0 0 .49-.33l4.93-4.92-.66 2.2a1.19 1.19 0 0 0 0 .46c.033.152.098.296.19.42.098.121.216.223.35.3.14.07.294.11.45.12a1 1 0 0 0 .48-.09 1.14 1.14 0 0 0 .39-.29.98.98 0 0 0 .22-.44c.032-.145.035-.294.01-.44zm-8-9.33a.46.46 0 0 1 0-.2.52.52 0 0 1 .12-.17.64.64 0 0 1 .18-.1.5.5 0 0 1 .21 0 .5.5 0 0 1 .32.15.5.5 0 0 1 .12.33v1.26l-1 1 .05-2.27zm1 11.35a.36.36 0 0 1-.16.11.47.47 0 0 1-.38 0 .361.361 0 0 1-.16-.11l-3.63-3.62a.5.5 0 0 1 0-.71l4.35-4.35v2.85a.74.74 0 0 0-.24.55.75.75 0 1 0 1.17-.55v-2.83l3.85 3.87-4.8 4.79z"
        /> < title > { title } < / title > < / svg >
    }
}
