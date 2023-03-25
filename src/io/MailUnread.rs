#[cfg(feature = "IoMailUnread")]
use leptos::*;
#[cfg(feature = "IoMailUnread")]
///This icon requires the feature `IoMailUnread` to be enabled.
#[component]
pub fn MailUnread(
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
        "M496,128.05A64,64,0,0,0,389.62,80h0a64.52,64.52,0,0,0-12.71,15.3l0,.06c-.54.9-1.05,1.82-1.55,2.74l-.24.49c-.42.79-.81,1.59-1.19,2.4-.12.25-.23.5-.34.75-.33.73-.65,1.47-.95,2.22-.13.31-.25.62-.37.93-.27.7-.53,1.4-.78,2.11l-.36,1.06c-.22.68-.43,1.37-.63,2.06-.12.39-.23.77-.33,1.16-.19.67-.35,1.35-.51,2-.1.41-.2.82-.29,1.23-.14.68-.27,1.37-.39,2-.08.42-.16.84-.23,1.26-.11.7-.2,1.41-.29,2.12-.05.41-.11.82-.16,1.24-.08.77-.13,1.54-.19,2.32,0,.36-.06.72-.08,1.08-.06,1.14-.1,2.28-.1,3.44h0c0,1,0,2,.08,2.94l0,.64q.08,1.41.21,2.82l.06.48c.09.85.19,1.69.32,2.52,0,.17,0,.35.07.52.14.91.31,1.81.49,2.71,0,.22.09.43.13.65.18.86.38,1.72.6,2.57,0,.07,0,.13,0,.19.23.89.48,1.76.75,2.63l.21.68c.27.85.55,1.68.85,2.51.06.18.13.36.2.54.27.71.55,1.42.84,2.12.08.21.16.41.25.61.34.79.69,1.58,1.06,2.36l.33.67c.35.7.7,1.4,1.07,2.09a64.34,64.34,0,0,0,22.14,23.81h0a62.22,62.22,0,0,0,7.62,4.15l.39.18q2.66,1.2,5.43,2.16l.95.32,1.5.47c.45.14.9.26,1.36.39l1.92.5,1.73.4,1.15.23,1.83.33.94.15c.9.13,1.81.25,2.72.35l.77.07c.73.06,1.47.12,2.21.16l.86.05c1,0,1.94.08,2.92.08h0c1.16,0,2.3,0,3.44-.1l1.08-.08c.78-.06,1.55-.11,2.32-.19l1.25-.16c.7-.09,1.41-.18,2.11-.29l1.26-.23c.68-.12,1.37-.25,2-.39l1.23-.29c.68-.16,1.36-.32,2-.51.39-.1.77-.21,1.16-.33.69-.2,1.38-.41,2.06-.63l1.06-.36c.71-.25,1.41-.51,2.11-.78l.93-.37c.75-.3,1.49-.62,2.22-.95l.75-.34c.81-.38,1.61-.77,2.4-1.19l.49-.24c.92-.5,1.84-1,2.74-1.55l.06,0A64.52,64.52,0,0,0,480,170.38h0A63.81,63.81,0,0,0,496,128.05Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M371.38,202.53l-105.56,82.1a16,16,0,0,1-19.64,0l-144-112a16,16,0,1,1,19.64-25.26L256,251.73l94.22-73.28A95.86,95.86,0,0,1,348.81,80H88a56.06,56.06,0,0,0-56,56V376a56.06,56.06,0,0,0,56,56H424a56.06,56.06,0,0,0,56-56V211.19a95.85,95.85,0,0,1-108.62-8.66Z"
        /> < title > { title } < / title > < / svg >
    }
}
