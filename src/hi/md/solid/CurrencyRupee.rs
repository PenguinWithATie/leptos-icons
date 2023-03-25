#[cfg(feature = "HiMdSolidCurrencyRupee")]
use leptos::*;
#[cfg(feature = "HiMdSolidCurrencyRupee")]
///This icon requires the feature `HiMdSolidCurrencyRupee` to be enabled.
#[component]
pub fn CurrencyRupee(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM6.00002 5.75C6.00002 5.33579 6.33581 5 6.75002 5H13.25C13.6642 5 14 5.33579 14 5.75C14 6.16421 13.6642 6.5 13.25 6.5H11.1227C11.5229 6.99922 11.8059 7.59647 11.9298 8.25H13.25C13.6642 8.25 14 8.58579 14 9C14 9.41421 13.6642 9.75 13.25 9.75H11.9298C11.6108 11.4316 10.2393 12.7405 8.52636 12.9657L10.2804 14.7197C10.5732 15.0126 10.5732 15.4874 10.2804 15.7803C9.98746 16.0732 9.51259 16.0732 9.21969 15.7803L6.21969 12.7803C6.00519 12.5658 5.94103 12.2432 6.05711 11.963C6.1732 11.6827 6.44668 11.5 6.75002 11.5H8.00002C9.11943 11.5 10.067 10.7643 10.3856 9.75H6.75C6.33579 9.75 6 9.41421 6 9C6 8.58579 6.33579 8.25 6.75 8.25H10.3856C10.067 7.23572 9.11943 6.5 8.00002 6.5H6.75002C6.33581 6.5 6.00002 6.16421 6.00002 5.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
