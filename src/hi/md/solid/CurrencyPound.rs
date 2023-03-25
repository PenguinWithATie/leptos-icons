#[cfg(feature = "HiMdSolidCurrencyPound")]
use leptos::*;
#[cfg(feature = "HiMdSolidCurrencyPound")]
///This icon requires the feature `HiMdSolidCurrencyPound` to be enabled.
#[component]
pub fn CurrencyPound(
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
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8.73223 6.23223C9.70854 5.25592 11.2915 5.25592 12.2678 6.23223C12.5607 6.52513 13.0355 6.52513 13.3284 6.23223C13.6213 5.93934 13.6213 5.46447 13.3284 5.17157C11.7663 3.60948 9.23367 3.60948 7.67157 5.17157C6.89067 5.95247 6.5 6.97747 6.5 8V8.16481C6.5 8.52945 6.53382 8.89272 6.60072 9.25H6.25C5.83579 9.25 5.5 9.58579 5.5 10C5.5 10.4142 5.83579 10.75 6.25 10.75H6.98724C7.175 11.782 7.05016 12.853 6.61952 13.8219L6.56464 13.9454C6.44963 14.2042 6.49018 14.5057 6.66948 14.7249C6.84878 14.9441 7.13625 15.0436 7.4127 14.9821L8.68531 14.6993C9.21441 14.5818 9.76361 14.589 10.2894 14.7204C11.0842 14.9191 11.9158 14.9191 12.7106 14.7204L13.6819 14.4776C14.0837 14.3771 14.3281 13.9699 14.2276 13.5681C14.1271 13.1663 13.7199 12.9219 13.3181 13.0224L12.3468 13.2652C11.7908 13.4042 11.2092 13.4042 10.6532 13.2652C9.91296 13.0802 9.14026 13.0673 8.39452 13.2275C8.58441 12.4159 8.62237 11.5757 8.5063 10.75H9.75C10.1642 10.75 10.5 10.4142 10.5 10C10.5 9.58579 10.1642 9.25 9.75 9.25H8.13603C8.0458 8.89574 8 8.53121 8 8.16481V8C8 7.35903 8.24393 6.72054 8.73223 6.23223Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
