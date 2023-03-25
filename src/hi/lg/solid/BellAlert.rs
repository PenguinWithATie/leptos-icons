#[cfg(feature = "HiLgSolidBellAlert")]
use leptos::*;
#[cfg(feature = "HiLgSolidBellAlert")]
///This icon requires the feature `HiLgSolidBellAlert` to be enabled.
#[component]
pub fn BellAlert(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.85036 3.50019C6.1266 3.19155 6.10034 2.7174 5.79169 2.44116C5.48305 2.16491 5.0089 2.19118 4.73266 2.49982C3.5317 3.84163 2.69605 5.51948 2.3846 7.37592C2.31607 7.78442 2.59167 8.17114 3.00017 8.23967C3.40868 8.3082 3.79539 8.0326 3.86393 7.6241C4.12704 6.05581 4.83301 4.63686 5.85036 3.50019Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.267 2.49982C18.9908 2.19118 18.5166 2.16491 18.208 2.44116C17.8993 2.7174 17.873 3.19155 18.1493 3.50019C19.1666 4.63686 19.8726 6.05581 20.1357 7.6241C20.2043 8.0326 20.591 8.3082 20.9995 8.23967C21.408 8.17114 21.6836 7.78442 21.6151 7.37592C21.3036 5.51948 20.4679 3.84163 19.267 2.49982Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M12 2.25C8.27216 2.25 5.25012 5.27197 5.25001 8.9998L5.24981 9.75C5.24981 11.8731 4.4488 13.8074 3.13126 15.2699C2.96476 15.4547 2.90073 15.71 2.96033 15.9516C3.01992 16.1931 3.19539 16.3893 3.42875 16.4755C4.97287 17.0455 6.58934 17.4659 8.2604 17.7192C8.25351 17.812 8.25001 17.9056 8.25001 18C8.25001 20.0711 9.92894 21.75 12 21.75C14.0711 21.75 15.75 20.0711 15.75 18C15.75 17.9056 15.7465 17.812 15.7396 17.7192C17.4105 17.4659 19.0269 17.0455 20.5709 16.4755C20.8042 16.3893 20.9797 16.1931 21.0393 15.9516C21.0989 15.71 21.0349 15.4547 20.8684 15.2699C19.5508 13.8074 18.7498 11.8731 18.7498 9.75V9.04919L18.75 9C18.75 5.27208 15.7279 2.25 12 2.25ZM9.75001 18C9.75001 17.9662 9.75075 17.9326 9.75221 17.8993C10.4927 17.966 11.2424 18 11.9998 18C12.7574 18 13.5072 17.9659 14.2478 17.8992C14.2493 17.9326 14.25 17.9662 14.25 18C14.25 19.2426 13.2427 20.25 12 20.25C10.7574 20.25 9.75001 19.2426 9.75001 18Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
