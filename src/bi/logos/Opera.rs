#[cfg(feature = "BiLogosOpera")]
use leptos::*;
#[cfg(feature = "BiLogosOpera")]
///This icon requires the feature `BiLogosOpera` to be enabled.
#[component]
pub fn Opera(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8.71 6.366C7.604 7.67 6.888 9.602 6.838 11.765v.471c.05 2.165.766 4.094 1.872 5.397 1.434 1.865 3.564 3.046 5.948 3.046a7.218 7.218 0 0 0 4.006-1.225 9.943 9.943 0 0 1-7.139 2.533A9.995 9.995 0 0 1 2 12C2 6.477 6.478 2 12 2h.038a9.971 9.971 0 0 1 6.627 2.545c-1.173-.773-2.543-1.225-4.009-1.225-2.382 0-4.514 1.184-5.95 3.046h.004zM22 12a9.971 9.971 0 0 1-3.335 7.455c-2.564 1.25-4.954.375-5.747-.172 2.52-.553 4.422-3.6 4.422-7.283 0-3.686-1.901-6.73-4.422-7.283.792-.545 3.183-1.42 5.747-.172A9.971 9.971 0 0 1 22 12z"
        /> < title > { title } < / title > < / svg >
    }
}
