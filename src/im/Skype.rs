#[cfg(feature = "ImSkype")]
use leptos::*;
#[cfg(feature = "ImSkype")]
///This icon requires the feature `ImSkype` to be enabled.
#[component]
pub fn Skype(
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
        "M6.65 0.584c-0.025-0.016-0.053-0.028-0.078-0.041-0.028 0.006-0.053 0.009-0.081 0.016l0.159 0.025z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M0.575 6.578c-0.006 0.028-0.009 0.056-0.012 0.081 0.016 0.025 0.025 0.050 0.041 0.075l-0.028-0.156z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.419 9.416c0.006-0.028 0.009-0.056 0.016-0.084-0.016-0.025-0.025-0.050-0.041-0.075l0.025 0.159z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9.25 15.359c0.025 0.016 0.053 0.028 0.078 0.041 0.028-0.006 0.056-0.009 0.084-0.012l-0.162-0.028z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.434 9.331c-0.006 0.028-0.009 0.056-0.016 0.084l-0.028-0.162c0.016 0.028 0.028 0.053 0.044 0.078 0.081-0.45 0.125-0.909 0.125-1.369 0-1.019-0.2-2.009-0.594-2.941-0.381-0.9-0.925-1.709-1.619-2.403s-1.503-1.238-2.4-1.619c-0.931-0.394-1.922-0.594-2.941-0.594-0.481 0-0.963 0.044-1.431 0.134 0 0-0.003 0-0.003 0 0.025 0.012 0.053 0.025 0.078 0.041l-0.159-0.025c0.028-0.006 0.053-0.009 0.081-0.016-0.644-0.341-1.366-0.525-2.097-0.525-1.194 0-2.319 0.466-3.163 1.309s-1.309 1.969-1.309 3.163c0 0.759 0.197 1.509 0.563 2.169 0.006-0.028 0.009-0.056 0.012-0.081l0.028 0.159c-0.016-0.025-0.028-0.050-0.041-0.075-0.075 0.428-0.112 0.866-0.112 1.303 0 1.019 0.2 2.009 0.594 2.941 0.381 0.9 0.925 1.706 1.619 2.4s1.503 1.238 2.403 1.619c0.931 0.394 1.922 0.594 2.941 0.594 0.444 0 0.887-0.041 1.322-0.119-0.025-0.016-0.050-0.028-0.078-0.041l0.162 0.028c-0.028 0.006-0.056 0.009-0.084 0.012 0.669 0.378 1.428 0.581 2.2 0.581 1.194 0 2.319-0.466 3.162-1.309s1.309-1.969 1.309-3.162c-0.003-0.759-0.2-1.509-0.569-2.175zM8.034 12.591c-2.684 0-3.884-1.319-3.884-2.309 0-0.506 0.375-0.863 0.891-0.863 1.15 0 0.85 1.65 2.994 1.65 1.097 0 1.703-0.597 1.703-1.206 0-0.366-0.181-0.772-0.903-0.95l-2.388-0.597c-1.922-0.481-2.272-1.522-2.272-2.5 0-2.028 1.909-2.791 3.703-2.791 1.653 0 3.6 0.913 3.6 2.131 0 0.522-0.453 0.825-0.969 0.825-0.981 0-0.8-1.356-2.775-1.356-0.981 0-1.522 0.444-1.522 1.078s0.775 0.838 1.447 0.991l1.769 0.394c1.934 0.431 2.425 1.563 2.425 2.625 0 1.647-1.266 2.878-3.819 2.878z"
        /> < title > { title } < / title > < / svg >
    }
}
