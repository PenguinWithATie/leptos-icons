#[cfg(feature = "AiOutlinedEuro")]
use leptos::*;
#[cfg(feature = "AiOutlinedEuro")]
///This icon requires the feature `AiOutlinedEuro` to be enabled.
#[component]
pub fn Euro(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372zm117.7-588.6c-15.9-3.5-34.4-5.4-55.3-5.4-106.7 0-178.9 55.7-198.6 149.9H344c-4.4 0-8 3.6-8 8v27.2c0 4.4 3.6 8 8 8h26.4c-.3 4.1-.3 8.4-.3 12.8v36.9H344c-4.4 0-8 3.6-8 8V568c0 4.4 3.6 8 8 8h30.2c17.2 99.2 90.4 158 200.2 158 20.9 0 39.4-1.7 55.3-5.1 3.7-.8 6.4-4 6.4-7.8v-42.8c0-5-4.6-8.8-9.5-7.8-14.7 2.8-31.9 4.1-51.8 4.1-68.5 0-114.5-36.6-129.8-98.6h130.6c4.4 0 8-3.6 8-8v-27.2c0-4.4-3.6-8-8-8H439.2v-36c0-4.7 0-9.4.3-13.8h135.9c4.4 0 8-3.6 8-8v-27.2c0-4.4-3.6-8-8-8H447.1c17.2-56.9 62.3-90.4 127.6-90.4 19.9 0 37.1 1.5 51.7 4.4a8 8 0 0 0 9.6-7.8v-42.8c0-3.8-2.6-7-6.3-7.8z"
        /> < title > { title } < / title > < / svg >
    }
}
