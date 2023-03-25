#[cfg(feature = "ImAmazon")]
use leptos::*;
#[cfg(feature = "ImAmazon")]
///This icon requires the feature `ImAmazon` to be enabled.
#[component]
pub fn Amazon(
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
        "M14.463 13.831c-1.753 1.294-4.291 1.981-6.478 1.981-3.066 0-5.825-1.131-7.912-3.019-0.163-0.147-0.019-0.35 0.178-0.234 2.253 1.313 5.041 2.1 7.919 2.1 1.941 0 4.075-0.403 6.041-1.238 0.294-0.125 0.544 0.197 0.253 0.409z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.191 13c-0.225-0.287-1.481-0.137-2.047-0.069-0.172 0.019-0.197-0.128-0.044-0.238 1.003-0.703 2.647-0.5 2.838-0.266 0.194 0.238-0.050 1.884-0.991 2.672-0.144 0.122-0.281 0.056-0.219-0.103 0.216-0.528 0.688-1.709 0.463-1.997z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.053 11.838l0.003 0.003c0.387-0.341 1.084-0.95 1.478-1.278 0.156-0.125 0.128-0.334 0.006-0.509-0.353-0.488-0.728-0.884-0.728-1.784v-3c0-1.272 0.088-2.438-0.847-3.313-0.738-0.706-1.963-0.956-2.9-0.956-1.831 0-3.875 0.684-4.303 2.947-0.047 0.241 0.131 0.369 0.287 0.403l1.866 0.203c0.175-0.009 0.3-0.181 0.334-0.356 0.159-0.778 0.813-1.156 1.547-1.156 0.397 0 0.847 0.144 1.081 0.5 0.269 0.397 0.234 0.938 0.234 1.397v0.25c-1.116 0.125-2.575 0.206-3.619 0.666-1.206 0.522-2.053 1.584-2.053 3.147 0 2 1.259 3 2.881 3 1.369 0 2.116-0.322 3.172-1.403 0.35 0.506 0.463 0.753 1.103 1.284 0.147 0.078 0.328 0.072 0.456-0.044zM9.113 7.144c0 0.75 0.019 1.375-0.359 2.041-0.306 0.544-0.791 0.875-1.331 0.875-0.737 0-1.169-0.563-1.169-1.394 0-1.641 1.472-1.938 2.863-1.938v0.416z"
        /> < title > { title } < / title > < / svg >
    }
}
