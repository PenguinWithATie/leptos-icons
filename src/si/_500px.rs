#[cfg(feature = "Si500px")]
use leptos::*;
#[cfg(feature = "Si500px")]
///This icon requires the feature `Si500px` to be enabled.
#[component]
pub fn _500px(
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
        "M7.433 9.01A2.994 2.994 0 0 0 4.443 12a2.993 2.993 0 0 0 2.99 2.99 2.994 2.994 0 0 0 2.99-2.99 2.993 2.993 0 0 0-2.99-2.99m0 5.343A2.357 2.357 0 0 1 5.079 12a2.357 2.357 0 0 1 2.354-2.353A2.356 2.356 0 0 1 9.786 12a2.356 2.356 0 0 1-2.353 2.353m6.471-5.343a2.994 2.994 0 0 0-2.99 2.99 2.993 2.993 0 0 0 2.99 2.99 2.994 2.994 0 0 0 2.99-2.99 2.994 2.994 0 0 0-2.99-2.99m0 5.343A2.355 2.355 0 0 1 11.552 12a2.355 2.355 0 0 1 2.352-2.353A2.356 2.356 0 0 1 16.257 12a2.356 2.356 0 0 1-2.353 2.353m-11.61-3.55a2.1 2.1 0 0 0-1.597.423V9.641h2.687c.093 0 .16-.017.16-.292 0-.269-.108-.28-.18-.28H.39c-.174 0-.265.14-.265.294v2.602c0 .136.087.183.247.214.141.028.223.012.285-.057l.006-.01c.283-.408.9-.804 1.486-.732.699.086 1.262.644 1.34 1.327a1.512 1.512 0 0 1-1.5 1.685c-.636 0-1.19-.408-1.422-1.001-.035-.088-.092-.152-.343-.062-.229.083-.243.18-.212.268a2.11 2.11 0 0 0 1.976 1.386 2.102 2.102 0 0 0 .305-4.18M18.938 9.04c-.805.062-1.434.77-1.434 1.61v2.66c0 .155.117.187.293.187s.293-.031.293-.186v-2.668c0-.524.382-.974.868-1.024a.972.972 0 0 1 .758.247.984.984 0 0 1 .322.73c0 .08-.039.34-.217.58-.135.182-.39.399-.844.399h-.009c-.115 0-.215.005-.234.28-.013.186-.012.269.148.29.286.04.576-.016.865-.166.492-.256.822-.741.861-1.267a1.562 1.562 0 0 0-.452-1.222 1.56 1.56 0 0 0-1.218-.45m3.919 1.56l1.085-1.086c.04-.039.132-.132-.055-.324-.08-.083-.153-.125-.217-.125h-.001a.163.163 0 0 0-.121.058L22.46 10.21l-1.086-1.093c-.088-.088-.19-.067-.322.065-.135.136-.157.24-.069.328l1.086 1.092-1.064 1.064-.007.007c-.026.025-.065.063-.065.125-.001.063.042.139.126.223.07.071.138.107.2.107.069 0 .114-.045.139-.07l1.068-1.067 1.09 1.092a.162.162 0 0 0 .115.045h.002c.069 0 .142-.04.217-.118.122-.129.143-.236.06-.319z"
        /> < title > { title } < / title > < / svg >
    }
}
