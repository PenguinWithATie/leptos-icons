#[cfg(feature = "FaSolidBowlRice")]
use leptos::*;
#[cfg(feature = "FaSolidBowlRice")]
///This icon requires the feature `FaSolidBowlRice` to be enabled.
#[component]
pub fn BowlRice(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M176 56c0-13.3 10.7-24 24-24h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H200c-13.3 0-24-10.7-24-24zm24 48h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H200c-13.3 0-24-10.7-24-24s10.7-24 24-24zM56 176H72c13.3 0 24 10.7 24 24s-10.7 24-24 24H56c-13.3 0-24-10.7-24-24s10.7-24 24-24zM0 283.4C0 268.3 12.3 256 27.4 256H484.6c15.1 0 27.4 12.3 27.4 27.4c0 70.5-44.4 130.7-106.7 154.1L403.5 452c-2 16-15.6 28-31.8 28H140.2c-16.1 0-29.8-12-31.8-28l-1.8-14.4C44.4 414.1 0 353.9 0 283.4zM224 200c0-13.3 10.7-24 24-24h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H248c-13.3 0-24-10.7-24-24zm-96 0c0-13.3 10.7-24 24-24h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H152c-13.3 0-24-10.7-24-24zm-24-96h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H104c-13.3 0-24-10.7-24-24s10.7-24 24-24zm216 96c0-13.3 10.7-24 24-24h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H344c-13.3 0-24-10.7-24-24zm-24-96h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H296c-13.3 0-24-10.7-24-24s10.7-24 24-24zm120 96c0-13.3 10.7-24 24-24h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H440c-13.3 0-24-10.7-24-24zm-24-96h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H392c-13.3 0-24-10.7-24-24s10.7-24 24-24zM296 32h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H296c-13.3 0-24-10.7-24-24s10.7-24 24-24z"
        /> < title > { title } < / title > < / svg >
    }
}
