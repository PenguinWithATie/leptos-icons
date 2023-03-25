#[cfg(feature = "HiMdSolidPhone")]
use leptos::*;
#[cfg(feature = "HiMdSolidPhone")]
///This icon requires the feature `HiMdSolidPhone` to be enabled.
#[component]
pub fn Phone(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 3.5C2 2.67157 2.67157 2 3.5 2H4.64837C5.35142 2 5.96014 2.4883 6.11265 3.1746L6.82887 6.39757C7.00105 7.17238 6.53984 7.9472 5.77667 8.16525L4.84388 8.43176C4.4331 8.54913 4.20107 8.98665 4.36396 9.38159C5.53001 12.2087 7.79126 14.47 10.6184 15.636C11.0134 15.7989 11.4509 15.5669 11.5682 15.1561L11.8348 14.2233C12.0528 13.4602 12.8276 12.999 13.6024 13.1711L16.8254 13.8873C17.5117 14.0399 18 14.6486 18 15.3516V16.5C18 17.3284 17.3284 18 16.5 18H15C13.8514 18 12.7366 17.8509 11.6742 17.5705C7.16665 16.3809 3.61908 12.8334 2.42949 8.32576C2.14913 7.26341 2 6.14856 2 5V3.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
