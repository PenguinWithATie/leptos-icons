#[cfg(feature = "TiSocialLastFm")]
use leptos::*;
#[cfg(feature = "TiSocialLastFm")]
///This icon requires the feature `TiSocialLastFm` to be enabled.
#[component]
pub fn SocialLastFm(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.942 16.182c2.374 0 3.558-.791 3.558-2.373 0-1.304-.749-2.132-2.254-2.49l-1.119-.235c-.637-.159-.951-.495-.951-1.009 0-.594.396-.889 1.186-.889.869 0 1.323.334 1.363 1.006l1.717-.178c-.114-1.463-1.109-2.195-2.962-2.195-2.019 0-3.026.832-3.026 2.495 0 1.182.654 1.935 1.958 2.251l1.188.236c.79.196 1.186.555 1.186 1.068 0 .631-.614.949-1.842.949-1.498 0-2.489-.732-2.962-2.195l-.597-1.721c-.354-1.145-.796-1.947-1.334-2.401-.53-.45-1.367-.683-2.519-.683-1.069 0-2.007.396-2.815 1.188-.811.791-1.217 1.838-1.217 3.142 0 1.223.383 2.203 1.156 2.935.774.733 1.688 1.099 2.756 1.099 1.069 0 1.918-.256 2.55-.77l-.53-1.485c-.554.556-1.211.833-1.96.833-.63 0-1.175-.248-1.628-.744-.455-.492-.686-1.137-.686-1.927 0-.989.247-1.708.743-2.163.497-.455 1.056-.681 1.689-.681.674 0 1.155.177 1.457.53.296.357.56.912.797 1.662l.537 1.721c.632 2.014 2.154 3.024 4.561 3.024"
        /> < title > { title } < / title > < / svg >
    }
}
