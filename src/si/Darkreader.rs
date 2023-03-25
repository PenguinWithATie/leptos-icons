#[cfg(feature = "SiDarkreader")]
use leptos::*;
#[cfg(feature = "SiDarkreader")]
///This icon requires the feature `SiDarkreader` to be enabled.
#[component]
pub fn Darkreader(
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
        "M18.281 8.572c-.18-1.671-.926-3.132-2.105-4.173A6.315 6.315 0 0012 2.824c-1.538 0-3.026.56-4.176 1.575C6.646 5.44 5.9 6.901 5.72 8.572a4.968 4.968 0 01.987-.101 4.587 4.587 0 014.24 2.827l2.107-.002a4.57 4.57 0 014.241-2.825 4.88 4.88 0 01.987.101zM3.624 16.494l-2.212 6.094H0l2.662-7.324a4.621 4.621 0 01-.401-1.046 4.803 4.803 0 01-.143-1.16 4.7 4.7 0 01.574-2.283 4.43 4.43 0 011.576-1.642c.08-2.207.943-4.178 2.43-5.593A7.7 7.7 0 0112 1.412c1.973 0 3.876.768 5.305 2.13 1.486 1.417 2.348 3.388 2.427 5.596a4.42 4.42 0 011.576 1.64c.383.693.576 1.478.574 2.28 0 .39-.047.78-.142 1.159-.091.362-.225.713-.402 1.045L24 22.588h-1.412l-2.212-6.097c-.41.367-.879.649-1.383.843a4.653 4.653 0 01-1.699.313 4.635 4.635 0 01-3.132-1.227c-.827-.765-1.344-1.814-1.443-3.008H11.28c-.103 1.192-.62 2.241-1.447 3.005a4.637 4.637 0 01-3.128 1.23 4.644 4.644 0 01-1.698-.31 4.514 4.514 0 01-1.384-.843zm11.2-3.445a2.462 2.462 0 002.489 2.48 2.47 2.47 0 00-.019-4.94 2.464 2.464 0 00-2.47 2.46zm-10.589.01a2.463 2.463 0 002.47 2.47 2.469 2.469 0 002.472-2.47 2.469 2.469 0 00-2.471-2.47 2.463 2.463 0 00-2.47 2.47zm5.647 6c.033-.423.327-.703.706-.706a.681.681 0 01.706.706v2.823a.681.681 0 01-.706.706c-.38-.003-.673-.283-.706-.706V19.06zm2.824 0c.033-.423.326-.703.706-.706a.681.681 0 01.706.706v2.823a.681.681 0 01-.706.706c-.38-.003-.673-.283-.706-.706V19.06zm2.823 1.412c.033-.423.327-.703.706-.706a.681.681 0 01.706.706v1.411a.681.681 0 01-.706.706c-.38-.003-.673-.283-.706-.706v-1.411zm-8.47 0c.033-.423.326-.703.706-.706a.681.681 0 01.706.706v1.411a.681.681 0 01-.706.706c-.38-.003-.673-.283-.706-.706v-1.411z"
        /> < title > { title } < / title > < / svg >
    }
}
