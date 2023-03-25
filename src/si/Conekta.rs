#[cfg(feature = "SiConekta")]
use leptos::*;
#[cfg(feature = "SiConekta")]
///This icon requires the feature `SiConekta` to be enabled.
#[component]
pub fn Conekta(
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
        "M12.2914 17.8831a11.7327 11.7327 0 0 1-6.1742 3.0338 2.4598 2.4598 0 0 1-2.1647-.7461c-4.2466-4.6258-4.2466-11.7322 0-16.358a2.4599 2.4599 0 0 1 2.1729-.7461 11.668 11.668 0 0 1 6.289 3.1404 27.1655 27.1655 0 0 0-.6969 6.1004 27.7762 27.7762 0 0 0 .5739 5.5756zm9.8962-3.9376a1.394 1.394 0 0 0-1.5244.5266 24.6804 24.6804 0 0 1-11.9139 8.9375 35.4417 35.4417 0 0 0 6.4284.5903 36.2857 36.2857 0 0 0 4.4605-.2788 3.5997 3.5997 0 0 0 3.0338-2.6977c.4692-1.884.6453-3.8838.5166-5.8134a1.394 1.394 0 0 0-1.001-1.2645zm-1.5245-4.3356a1.394 1.394 0 0 0 2.5255-.7462c.1354-1.9699-.0438-3.9689-.5166-5.8872A3.5997 3.5997 0 0 0 19.6382.2789 36.2678 36.2678 0 0 0 15.1776 0a35.4337 35.4337 0 0 0-6.4284.5904 24.6396 24.6396 0 0 1 11.9139 9.0195z"
        /> < title > { title } < / title > < / svg >
    }
}
