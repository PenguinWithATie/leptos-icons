#[cfg(feature = "FaSolidCloudMoonRain")]
use leptos::*;
#[cfg(feature = "FaSolidCloudMoonRain")]
///This icon requires the feature `FaSolidCloudMoonRain` to be enabled.
#[component]
pub fn CloudMoonRain(
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
        "M453 0C392.5 0 342.2 43.8 333 101.3c33.3 16.5 56.7 50.2 58.5 89.5c21.8 10.3 39.5 27.9 50 49.6c3.8 .4 7.7 .5 11.6 .5c32.9 0 62.8-13 84.6-34c4.8-4.6 6-11.7 3-17.6s-9.5-9.2-16-8.1c-4.6 .8-9.4 1.2-14.3 1.2c-46.1 0-83.2-37-83.2-82.3c0-30.6 17-57.5 42.3-71.6c5.8-3.2 8.8-9.9 7.4-16.3S469.9 .9 463.3 .4C459.9 .1 456.4 0 453 0zM346.4 361.3c41.6 0 75.3-33.7 75.3-75.3c0-37-26.7-67.8-62-74.1c1.2-5.2 1.8-10.7 1.8-16.2c0-41.6-33.7-75.3-75.3-75.3c-16 0-30.9 5-43.1 13.5c-15.8-26.2-44.5-43.7-77.4-43.7c-49.9 0-90.3 40.4-90.3 90.3l0 1.2C32.7 189.1 .1 226.2 .1 271c0 49.9 40.4 90.3 90.3 90.3H346.4zm-265.9 34c-10.4-6.9-24.4-4.1-31.3 6.3L19.1 446.8c-6.9 10.4-4.1 24.4 6.3 31.3s24.4 4.1 31.3-6.3l30.1-45.2c6.9-10.4 4.1-24.4-6.3-31.3zm90.3 0c-10.4-6.9-24.4-4.1-31.3 6.3l-30.1 45.2c-6.9 10.4-4.1 24.4 6.3 31.3s24.4 4.1 31.3-6.3l30.1-45.2c6.9-10.4 4.1-24.4-6.3-31.3zm90.3 0c-10.4-6.9-24.4-4.1-31.3 6.3l-30.1 45.2c-6.9 10.4-4.1 24.4 6.3 31.3s24.4 4.1 31.3-6.3l30.1-45.2c6.9-10.4 4.1-24.4-6.3-31.3zm90.3 0c-10.4-6.9-24.4-4.1-31.3 6.3l-30.1 45.2c-6.9 10.4-4.1 24.4 6.3 31.3s24.4 4.1 31.3-6.3l30.1-45.2c6.9-10.4 4.1-24.4-6.3-31.3z"
        /> < title > { title } < / title > < / svg >
    }
}
