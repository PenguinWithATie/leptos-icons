#[cfg(feature = "ImSteam2")]
use leptos::*;
#[cfg(feature = "ImSteam2")]
///This icon requires the feature `ImSteam2` to be enabled.
#[component]
pub fn Steam2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4.749 13.063c0.424 0 0.84-0.205 1.093-0.585 0.402-0.603 0.239-1.418-0.364-1.82l-1.032-0.688c0.177-0.048 0.362-0.074 0.554-0.074 1.162 0 2.105 0.942 2.105 2.105s-0.942 2.105-2.105 2.105c-1.131 0-2.054-0.893-2.102-2.012l1.124 0.749c0.224 0.149 0.477 0.221 0.727 0.221zM13.333 0c1.467 0 2.667 1.2 2.667 2.667v10.666c0 1.468-1.2 2.667-2.667 2.667h-10.666c-1.467 0-2.667-1.199-2.667-2.667v-3.172l1.896 1.264c-0.182 0.987 0.108 2.044 0.872 2.808 1.233 1.233 3.232 1.233 4.465 0 0.757-0.757 1.049-1.804 0.876-2.784l3.892-3.484c0.723-0.104 1.419-0.433 1.975-0.989 1.367-1.367 1.367-3.583 0-4.95s-3.583-1.367-4.95 0c-0.556 0.556-0.886 1.252-0.989 1.975v0l-3.198 4.847c-0.498 0.025-0.99 0.168-1.433 0.428l-3.404-2.269v-4.339c0-1.467 1.2-2.667 2.667-2.667h10.666zM14 4.5c0-1.381-1.119-2.5-2.5-2.5s-2.5 1.119-2.5 2.5 1.119 2.5 2.5 2.5 2.5-1.119 2.5-2.5zM10 4.5c0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5-0.672 1.5-1.5 1.5-1.5-0.672-1.5-1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
