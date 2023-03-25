#[cfg(feature = "HiMdSolidRocketLaunch")]
use leptos::*;
#[cfg(feature = "HiMdSolidRocketLaunch")]
///This icon requires the feature `HiMdSolidRocketLaunch` to be enabled.
#[component]
pub fn RocketLaunch(
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
        "M4.60597 12.9691C4.85946 13.2967 4.79938 13.7678 4.47179 14.0213C3.87925 14.4798 3.5 15.1954 3.5 16C3.5 16.1569 3.51437 16.31 3.54177 16.4582C3.68996 16.4856 3.84307 16.5 4 16.5C4.80458 16.5 5.52023 16.1208 5.97873 15.5282C6.23222 15.2006 6.70328 15.1405 7.03087 15.394C7.35846 15.6475 7.41854 16.1186 7.16505 16.4462C6.4347 17.39 5.2884 18 4 18C3.54003 18 3.09667 17.9221 2.6834 17.778C2.46722 17.7027 2.29728 17.5328 2.22195 17.3166C2.07794 16.9033 2 16.46 2 16C2 14.7116 2.60997 13.5653 3.55383 12.835C3.88142 12.5815 4.35248 12.6415 4.60597 12.9691Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5.75236 12C6.40714 12.8376 7.16237 13.5929 8 14.2476V18.25C8 18.6642 8.33579 19 8.75 19C11.5114 19 13.75 16.7614 13.75 14C13.75 13.5097 13.6792 13.035 13.5471 12.5861C16.8467 10.2297 19 6.36651 19 2C19 1.91234 18.9991 1.82486 18.9974 1.73759C18.9894 1.33502 18.665 1.01058 18.2624 1.0026C18.1751 1.00087 18.0877 1 18 1C13.6335 1 9.77032 3.15331 7.41387 6.45291C6.96496 6.32078 6.49028 6.25 6 6.25C3.23858 6.25 1 8.48858 1 11.25C1 11.6642 1.33579 12 1.75 12H5.75236ZM13 9C14.1046 9 15 8.10457 15 7C15 5.89543 14.1046 5 13 5C11.8954 5 11 5.89543 11 7C11 8.10457 11.8954 9 13 9Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
