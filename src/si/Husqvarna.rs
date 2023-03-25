#[cfg(feature = "SiHusqvarna")]
use leptos::*;
#[cfg(feature = "SiHusqvarna")]
///This icon requires the feature `SiHusqvarna` to be enabled.
#[component]
pub fn Husqvarna(
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
        "M12.001 14.457c2.04 0 1.997 0 1.997.645v4.054h2.775V7.927h-2.775v3.371c0 .664.042.662-1.997.662h.001c-2.04 0-1.996.002-1.996-.662v-3.37H7.23v11.228h2.775v-4.054c0-.644-.043-.645 1.996-.645M12 0c1.04 0 1.72.18 2.004.241-.143 1.307-.1 2.355 0 2.657.347 1.006 2.32 1.475 3.115 1.012.878-.523.676-2.109.676-3.155 0 0 1.294.301 2.086.594.644.238 1.593.703 1.593.703.188 1.323.01 3.281-1.316 3.945 1 .985 1.335 2.152 1.764 4.304.346 1.932.265 3.28.243 4.405-.101 1.53-.101 2.354-.468 3.743-.53 1.588-.844 2.335-1.918 3.438C17.599 24.131 14.045 24 12 23.998c-2.045.001-5.598.133-7.779-2.11-1.074-1.104-1.388-1.85-1.918-3.439-.367-1.389-.367-2.213-.469-3.743-.02-1.126-.102-2.473.245-4.405.428-2.152.763-3.319 1.763-4.304-1.326-.664-1.504-2.622-1.316-3.945 0 0 .95-.465 1.593-.703.792-.293 2.085-.594 2.085-.594 0 1.046-.201 2.632.676 3.155.797.463 2.768-.006 3.116-1.012.102-.302.142-1.35 0-2.657C10.28.181 10.959 0 12 0m-.004 21.125c2.387 0 3.149 0 4.331-.322.408-.14 1.143-.341 1.674-1.166.979-1.65.979-4.747.979-6.236 0-1.49 0-4.125-.979-5.754-.53-.846-1.255-1.078-1.674-1.188-1.208-.316-1.983-.297-4.331-.302h.002c-2.348.005-3.123-.014-4.331.302-.418.11-1.143.342-1.674 1.188-.979 1.63-.979 4.264-.979 5.754 0 1.489 0 4.586.98 6.236.53.825 1.264 1.025 1.673 1.166 1.182.323 1.944.322 4.33.322Z"
        /> < title > { title } < / title > < / svg >
    }
}
