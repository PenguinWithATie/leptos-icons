#[cfg(feature = "TiSortNumericallyOutline")]
use leptos::*;
#[cfg(feature = "TiSortNumericallyOutline")]
///This icon requires the feature `TiSortNumericallyOutline` to be enabled.
#[component]
pub fn SortNumericallyOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M23.292 12.134c.138-.445.208-.91.208-1.384 0-2.619-2.131-4.75-4.75-4.75-1.396 0-2.685.61-3.573 1.632l-.056-.067c-.973-.974-2.349-1.533-3.776-1.533-1.422 0-2.794.556-3.77 1.525-.264-.431-.644-.813-1.122-1.108-.474-.294-1.032-.449-1.613-.449-.482 0-.955.109-1.369.316l-1.406.747c-1.442.721-2.051 2.526-1.313 4.002.272.543.714.982 1.248 1.272v4.663c0 1.654 1.346 3 3 3 .766 0 1.458-.297 1.989-.771.54.487 1.25.771 2.011.771h5c.778 0 1.479-.305 2.01-.795.796.5 1.731.795 2.74.795 2.895 0 5.25-2.355 5.25-5.25 0-.922-.25-1.825-.708-2.616zm-17.292 4.866c0 .552-.448 1-1 1s-1-.448-1-1v-6.382c-.144.072-.306.106-.471.106-.401 0-.813-.203-.988-.553-.247-.494-.031-1.095.463-1.342l1.361-.724c.141-.07.307-.105.475-.105.199 0 .4.05.561.149.294.183.599.504.599.851v8zm8 1h-5c-.404 0-.769-.244-.924-.617-.155-.374-.069-.804.217-1.09l4-4c.254-.254.394-.591.394-.95 0-.358-.14-.695-.394-.949s-.601-.381-.949-.381-.696.127-.952.382c-.252.252-.392.589-.392.948 0 .552-.448 1-1 1s-1-.448-1-1c0-.894.348-1.733.98-2.364.632-.631 1.498-.947 2.364-.947s1.731.316 2.363.948c.632.631.979 1.471.979 2.363 0 .893-.348 1.733-.979 2.364l-2.293 2.293h2.586c.552 0 1 .448 1 1s-.448 1-1 1zm4.75 0c-1.792 0-3.25-1.458-3.25-3.25 0-.552.448-1 1-1s1 .448 1 1c0 .689.561 1.25 1.25 1.25s1.25-.561 1.25-1.25-.561-1.25-1.25-1.25c-.552 0-1-.448-1-1s.448-1 1-1c.414 0 .75-.336.75-.75s-.336-.75-.75-.75c-.281 0-.536.155-.665.404-.178.343-.527.54-.889.54-.155 0-.312-.036-.459-.112-.491-.254-.682-.857-.428-1.348.475-.915 1.41-1.484 2.441-1.484 1.516 0 2.75 1.233 2.75 2.75 0 .611-.207 1.17-.545 1.627.639.594 1.045 1.434 1.045 2.373 0 1.792-1.458 3.25-3.25 3.25z"
        /> < title > { title } < / title > < / svg >
    }
}
