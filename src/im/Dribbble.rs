#[cfg(feature = "ImDribbble")]
use leptos::*;
#[cfg(feature = "ImDribbble")]
///This icon requires the feature `ImDribbble` to be enabled.
#[component]
pub fn Dribbble(
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
        "M8 16c-4.412 0-8-3.588-8-8s3.587-8 8-8c4.412 0 8 3.587 8 8s-3.588 8-8 8v0zM14.747 9.094c-0.234-0.075-2.116-0.634-4.256-0.291 0.894 2.456 1.256 4.456 1.328 4.872 1.531-1.037 2.625-2.678 2.928-4.581v0zM10.669 14.3c-0.103-0.6-0.497-2.688-1.456-5.181-0.016 0.006-0.031 0.009-0.044 0.016-3.856 1.344-5.241 4.016-5.362 4.266 1.159 0.903 2.616 1.444 4.194 1.444 0.947 0 1.85-0.194 2.669-0.544v0zM2.922 12.578c0.156-0.266 2.031-3.369 5.553-4.509 0.088-0.028 0.178-0.056 0.269-0.081-0.172-0.388-0.359-0.778-0.553-1.159-3.409 1.022-6.722 0.978-7.022 0.975-0.003 0.069-0.003 0.138-0.003 0.209 0 1.753 0.666 3.356 1.756 4.566v0zM1.313 6.609c0.306 0.003 3.122 0.016 6.319-0.831-1.131-2.013-2.353-3.706-2.534-3.953-1.913 0.903-3.344 2.666-3.784 4.784v0zM6.4 1.366c0.188 0.253 1.431 1.944 2.55 4 2.431-0.909 3.459-2.294 3.581-2.469-1.206-1.072-2.794-1.722-4.531-1.722-0.55 0.003-1.088 0.069-1.6 0.191v0zM13.291 3.691c-0.144 0.194-1.291 1.663-3.816 2.694 0.159 0.325 0.313 0.656 0.453 0.991 0.050 0.119 0.1 0.234 0.147 0.353 2.275-0.284 4.534 0.172 4.759 0.219-0.016-1.612-0.594-3.094-1.544-4.256v0z"
        /> < title > { title } < / title > < / svg >
    }
}
