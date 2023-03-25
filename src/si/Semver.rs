#[cfg(feature = "SiSemver")]
use leptos::*;
#[cfg(feature = "SiSemver")]
///This icon requires the feature `SiSemver` to be enabled.
#[component]
pub fn Semver(
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
        "M.357 9.024A12.07 12.07 0 002.97 19.867a12.051 12.051 0 0010.38 4.063c7.768-.703 13.086-9.799 9.517-16.8-.416-1.19-2.07-.368-1.903.596.287.7.526 1.421.713 2.155a9.983 9.983 0 01-3.926 10.25 9.965 9.965 0 01-14.807-3.809A9.984 9.984 0 014.44 5.448a9.968 9.968 0 014.85-3.044 9.868 9.868 0 017.02.631.333.333 0 01.155.429l-3.962 10.62c-.107.81-.69.786-.797 0l-2.38-7.37a1.572 1.572 0 00-.773-.988c-1.19-.56-3.093.667-2.379 2.155l3.914 10.441c.524 1.393 1.023 1.834 2.058 1.834s1.535-.44 2.058-1.834L20 3.94a1.036 1.036 0 00-.369-1.19C13.1-2.907 2.32.641.357 9.023z"
        /> < title > { title } < / title > < / svg >
    }
}
