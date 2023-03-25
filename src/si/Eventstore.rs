#[cfg(feature = "SiEventstore")]
use leptos::*;
#[cfg(feature = "SiEventstore")]
///This icon requires the feature `SiEventstore` to be enabled.
#[component]
pub fn Eventstore(
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
        "M5.785 3.414c.428-.251.874-.473 1.339-.666.981-.421 1.995-.659 3.036-.765l1.127 1.274-.525 1.319c-1.02.195-1.979.57-2.879 1.11l-.195.12-.178-1.737-1.725-.66v.005zM5.295 3.724l1.789.69.188 1.71c-.969.713-1.725 1.623-2.264 2.73-.051.099-.096.198-.139.3l-1.26-1.529-2.054.434c.511-1.104 1.201-2.091 2.063-2.96.515-.522 1.07-.979 1.667-1.376l.01.001zm11.782 12.052c.195-.324.354-.652.48-.99l1.938 2.35 3.255-.701c-.187.513-.407 1.015-.662 1.507-.671 1.291-1.553 2.409-2.648 3.36-1.112.962-2.369 1.665-3.771 2.107-.027.009-.056.016-.083.026l-2.411-2.039.629-2.775.051-.021c1.379-.6 2.457-1.529 3.224-2.835v.011zM21.153 4.805c.199.273.384.56.563.854.741 1.248 1.254 2.582 1.539 4.004.271 1.41.315 2.829.12 4.252-.105.684-.255 1.35-.464 2.002l-3.255.699-1.95-2.357c.226-.867.255-1.77.06-2.709-.225-1.114-.72-2.065-1.454-2.85l3.509-.42 1.306-3.476h.026zM1.338 8.584l2.1-.444 1.236 1.509c-.391 1.084-.48 2.21-.285 3.38.045.334.119.656.21.969l-2.22-.505-1.59 1.881c-.193-.833-.283-1.685-.283-2.554.003-1.451.27-2.85.807-4.199l.016-.039.009.002zM4.78 14.518c.319.844.788 1.614 1.409 2.316.465.539.99.975 1.563 1.319l-2.2 1.261.12 2.864c-.726-.451-1.395-.99-2-1.605-1.021-1.05-1.8-2.249-2.34-3.6-.149-.375-.27-.75-.375-1.125l1.635-1.919 2.22.509-.032-.02zM8.232 18.415c.473.236.979.416 1.518.54 1.207.28 2.385.233 3.529-.141l-.625 2.757 2.387 2.02c-1.193.313-2.414.445-3.659.401-1.455-.046-2.853-.354-4.19-.925-.36-.153-.705-.322-1.041-.51l-.119-2.882 2.2-1.26zM20.815 4.364L19.502 7.85l-3.614.435c-.105-.091-.213-.181-.327-.255-1.185-.9-2.52-1.426-4.004-1.575-.346-.029-.675-.029-.99-.029l1.26-3.226L9.1.095c.827-.09 1.637-.104 2.459-.09.49.015.975.051 1.459.113.68.089 1.351.239 2.015.42 1.405.4 2.685 1.034 3.842 1.93.729.559 1.376 1.191 1.94 1.896z"
        /> < title > { title } < / title > < / svg >
    }
}
