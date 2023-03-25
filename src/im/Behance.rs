#[cfg(feature = "ImBehance")]
use leptos::*;
#[cfg(feature = "ImBehance")]
///This icon requires the feature `ImBehance` to be enabled.
#[component]
pub fn Behance(
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
        "M4.641 3.206c0.472 0 0.897 0.041 1.284 0.125 0.388 0.081 0.716 0.219 0.994 0.406 0.275 0.188 0.487 0.438 0.644 0.75 0.15 0.309 0.225 0.697 0.225 1.156 0 0.497-0.112 0.909-0.338 1.241-0.228 0.331-0.559 0.6-1.003 0.813 0.606 0.175 1.053 0.481 1.353 0.916 0.3 0.438 0.444 0.963 0.444 1.581 0 0.5-0.097 0.928-0.287 1.291-0.194 0.366-0.456 0.662-0.778 0.891-0.325 0.231-0.7 0.4-1.119 0.509-0.416 0.109-0.844 0.166-1.287 0.166h-4.772v-9.844h4.641zM4.359 7.181c0.384 0 0.703-0.091 0.953-0.275 0.25-0.181 0.369-0.481 0.369-0.894 0-0.228-0.041-0.419-0.122-0.566-0.084-0.147-0.194-0.263-0.334-0.344-0.138-0.084-0.294-0.141-0.478-0.172-0.178-0.034-0.366-0.050-0.556-0.050h-2.025v2.3h2.194zM4.478 11.372c0.213 0 0.416-0.019 0.606-0.063 0.194-0.044 0.366-0.109 0.509-0.209 0.144-0.097 0.266-0.225 0.353-0.394 0.088-0.166 0.128-0.378 0.128-0.637 0-0.506-0.144-0.869-0.428-1.088-0.284-0.216-0.662-0.322-1.131-0.322h-2.35v2.709h2.313z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.331 11.338c0.294 0.287 0.716 0.431 1.266 0.431 0.394 0 0.738-0.1 1.022-0.3s0.456-0.412 0.522-0.631h1.725c-0.278 0.859-0.697 1.469-1.272 1.838-0.566 0.369-1.259 0.556-2.063 0.556-0.563 0-1.066-0.091-1.519-0.269-0.453-0.181-0.831-0.434-1.15-0.766-0.309-0.331-0.553-0.725-0.725-1.188-0.169-0.459-0.256-0.969-0.256-1.519 0-0.534 0.088-1.031 0.262-1.491 0.178-0.463 0.422-0.859 0.747-1.194s0.706-0.6 1.156-0.794c0.447-0.194 0.941-0.291 1.488-0.291 0.603 0 1.131 0.116 1.584 0.353 0.45 0.234 0.822 0.55 1.113 0.944s0.497 0.847 0.625 1.353c0.128 0.506 0.172 1.034 0.137 1.588h-5.147c0 0.559 0.188 1.094 0.484 1.378zM13.578 7.594c-0.231-0.256-0.628-0.397-1.106-0.397-0.313 0-0.572 0.053-0.778 0.159-0.203 0.106-0.369 0.237-0.497 0.394-0.125 0.156-0.213 0.325-0.262 0.503-0.050 0.172-0.081 0.331-0.091 0.469h3.188c-0.047-0.5-0.219-0.869-0.453-1.128z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M10.444 4h3.991v0.972h-3.991v-0.972z" /> < title > { title } < / title > < / svg
        >
    }
}
