#[cfg(feature = "TiNotesOutline")]
use leptos::*;
#[cfg(feature = "TiNotesOutline")]
///This icon requires the feature `TiNotesOutline` to be enabled.
#[component]
pub fn NotesOutline(
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
        "M18.324 4.367c-.368-.324-.84-.5-1.324-.5l-.248.016-9 1.25c-1.001.125-1.752.975-1.752 1.984v6.111c-1.746.551-3 2.034-3 3.772 0 2.205 2.019 4 4.5 4 1.695 0 3.169-.842 3.937-2.078.803.667 1.879 1.078 3.063 1.078 2.481 0 4.5-1.795 4.5-4v-10.133c0-.574-.246-1.119-.676-1.5zm-7.324 11.633v-4.256l3-.45v1.737c-1.693.208-3 1.46-3 2.969zm6 0c0 1.104-1.119 2-2.5 2s-2.5-.896-2.5-2 1.119-2 2.5-2c.172 0 .338.014.5.041v-3.908l-5 .75v6.117c0 1.104-1.119 2-2.5 2s-2.5-.896-2.5-2 1.119-2 2.5-2c.172 0 .338.014.5.041v-7.924l9-1.25v10.133z"
        /> < title > { title } < / title > < / svg >
    }
}
