#[cfg(feature = "IoGlasses")]
use leptos::*;
#[cfg(feature = "IoGlasses")]
///This icon requires the feature `IoGlasses` to be enabled.
#[component]
pub fn Glasses(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,184H453.1a78.72,78.72,0,0,0-16-7.18C419.5,171,396.26,168,368,168s-51.5,3-69.06,8.82c-14.06,4.69-20.25,9.86-22.25,11.87h0a47.94,47.94,0,0,0-41.36,0h0c-2-2-8.19-7.18-22.25-11.87C195.5,171,172.26,168,144,168s-51.5,3-69.06,8.82a78.72,78.72,0,0,0-16,7.18H48a16,16,0,0,0,0,32h.17c1,45.46,6.44,72.78,18.11,92.23a66.78,66.78,0,0,0,31.92,28c12.23,5.24,27.22,7.79,45.8,7.79,24.15,0,58.48-3.71,77.72-35.77,9.68-16.14,15.09-37.69,17.21-70.52A16,16,0,0,0,240,232a16,16,0,0,1,32,0,16,16,0,0,0,1.07,5.71c2.12,32.83,7.53,54.38,17.21,70.52a66.78,66.78,0,0,0,31.92,28c12.23,5.24,27.22,7.79,45.8,7.79,24.15,0,58.48-3.71,77.72-35.77,11.67-19.45,17.13-46.77,18.11-92.23H464a16,16,0,0,0,0-32Z"
        /> < title > { title } < / title > < / svg >
    }
}
