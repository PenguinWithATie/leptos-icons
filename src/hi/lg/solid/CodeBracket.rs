#[cfg(feature = "HiLgSolidCodeBracket")]
use leptos::*;
#[cfg(feature = "HiLgSolidCodeBracket")]
///This icon requires the feature `HiLgSolidCodeBracket` to be enabled.
#[component]
pub fn CodeBracket(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M14.4473 3.02662C14.847 3.1356 15.0826 3.54791 14.9736 3.94753L10.4736 20.4475C10.3646 20.8471 9.95228 21.0827 9.55266 20.9738C9.15304 20.8648 8.91744 20.4525 9.02643 20.0529L13.5264 3.55285C13.6354 3.15323 14.0477 2.91763 14.4473 3.02662ZM16.7197 6.21986C17.0126 5.92696 17.4874 5.92696 17.7803 6.21986L23.0303 11.4699C23.3232 11.7628 23.3232 12.2376 23.0303 12.5305L17.7803 17.7805C17.4874 18.0734 17.0126 18.0734 16.7197 17.7805C16.4268 17.4876 16.4268 17.0128 16.7197 16.7199L21.4393 12.0002L16.7197 7.28052C16.4268 6.98762 16.4268 6.51275 16.7197 6.21986ZM7.28033 6.21986C7.57322 6.51275 7.57322 6.98763 7.28033 7.28052L2.56066 12.0002L7.28033 16.7199C7.57322 17.0128 7.57322 17.4876 7.28033 17.7805C6.98744 18.0734 6.51256 18.0734 6.21967 17.7805L0.96967 12.5305C0.676777 12.2376 0.676777 11.7628 0.96967 11.4699L6.21967 6.21986C6.51256 5.92697 6.98744 5.92697 7.28033 6.21986Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
