#[cfg(feature = "IoToday")]
use leptos::*;
#[cfg(feature = "IoToday")]
///This icon requires the feature `IoToday` to be enabled.
#[component]
pub fn Today(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M416,64H400V48.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,368,48V64H144V48.45c0-8.61-6.62-16-15.23-16.43A16,16,0,0,0,112,48V64H96a64,64,0,0,0-64,64v12a4,4,0,0,0,4,4H476a4,4,0,0,0,4-4V128A64,64,0,0,0,416,64Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M477,176H35a3,3,0,0,0-3,3V416a64,64,0,0,0,64,64H416a64,64,0,0,0,64-64V179A3,3,0,0,0,477,176ZM224,307.43A28.57,28.57,0,0,1,195.43,336H124.57A28.57,28.57,0,0,1,96,307.43V236.57A28.57,28.57,0,0,1,124.57,208h70.86A28.57,28.57,0,0,1,224,236.57Z"
        /> < title > { title } < / title > < / svg >
    }
}
