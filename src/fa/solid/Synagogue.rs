#[cfg(feature = "FaSolidSynagogue")]
use leptos::*;
#[cfg(feature = "FaSolidSynagogue")]
///This icon requires the feature `FaSolidSynagogue` to be enabled.
#[component]
pub fn Synagogue(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M309.8 3.7c5.9-4.9 14.6-4.9 20.5 0l121 100.8C469.5 119.7 480 142.2 480 166V280.1 512H464 352V416c0-17.7-14.3-32-32-32s-32 14.3-32 32v96H176 160V280.1 166c0-23.7 10.5-46.3 28.8-61.5L309.8 3.7zM512 512V244.5l28.1-31.2c3-3.4 7.4-5.3 11.9-5.3s8.9 1.9 11.9 5.3l63.8 70.9c7.9 8.8 12.3 20.3 12.3 32.1V448c0 35.3-28.7 64-64 64H512zM128 244.5V512H64c-35.3 0-64-28.7-64-64V316.3c0-11.9 4.4-23.3 12.3-32.1l63.8-70.9c3-3.4 7.4-5.3 11.9-5.3s8.9 1.9 11.9 5.3L128 244.5zM327 124.3c-3.1-5.4-10.9-5.4-13.9 0l-15.9 28.1-32.3-.3c-6.2-.1-10.1 6.7-7 12.1L274.3 192l-16.4 27.8c-3.2 5.4 .7 12.1 7 12.1l32.3-.3L313 259.7c3.1 5.4 10.9 5.4 13.9 0l15.9-28.1 32.3 .3c6.2 .1 10.1-6.7 7-12.1L365.7 192l16.4-27.8c3.2-5.4-.7-12.1-7-12.1l-32.3 .3L327 124.3z"
        /> < title > { title } < / title > < / svg >
    }
}
