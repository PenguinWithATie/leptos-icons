#[cfg(feature = "BsSignNoLeftTurn")]
use leptos::*;
#[cfg(feature = "BsSignNoLeftTurn")]
///This icon requires the feature `BsSignNoLeftTurn` to be enabled.
#[component]
pub fn SignNoLeftTurn(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sign-no-left-turn" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 8a8 8 0 1 0 16 0A8 8 0 0 0 0 8Zm3.416 5.29 5.988-5.987c.362.274.596.708.596 1.197V11h1V8.5c0-.765-.344-1.45-.885-1.908l3.176-3.176a7 7 0 0 1-9.874 9.874Zm-.707-.706a7 7 0 0 1 9.874-9.874L9.196 6.097A2.501 2.501 0 0 0 8.5 6H7V4.534a.25.25 0 0 0-.41-.192L4.23 6.308a.25.25 0 0 0 0 .384l2.36 1.966a.265.265 0 0 0 .026.02l-3.907 3.906ZM8.293 7 7 8.293V7h1.293Z"
        /> < title > { title } < / title > < / svg >
    }
}
