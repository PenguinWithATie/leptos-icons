#[cfg(feature = "HiLgSolidCamera")]
use leptos::*;
#[cfg(feature = "HiLgSolidCamera")]
///This icon requires the feature `HiLgSolidCamera` to be enabled.
#[component]
pub fn Camera(
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
        "M12 9C9.92893 9 8.25 10.6789 8.25 12.75C8.25 14.8211 9.92893 16.5 12 16.5C14.0711 16.5 15.75 14.8211 15.75 12.75C15.75 10.6789 14.0711 9 12 9Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9.34429 3.07113C10.2236 3.02391 11.109 3 12 3C12.891 3 13.7764 3.02391 14.6557 3.07113C15.6233 3.12309 16.4854 3.65612 16.9882 4.46184L17.8094 5.77786C18.0485 6.16099 18.4544 6.42131 18.9195 6.48741C19.3049 6.54218 19.6888 6.60145 20.0713 6.66518C21.5031 6.90378 22.5 8.15785 22.5 9.57403V18C22.5 19.6569 21.1569 21 19.5 21H4.5C2.84315 21 1.5 19.6569 1.5 18V9.57402C1.5 8.15784 2.49686 6.90377 3.92872 6.66517C4.31116 6.60144 4.6951 6.54217 5.08048 6.4874C5.54556 6.42131 5.95152 6.16099 6.19061 5.77785L7.01181 4.46184C7.5146 3.65612 8.37667 3.12309 9.34429 3.07113ZM6.75 12.75C6.75 9.8505 9.1005 7.5 12 7.5C14.8995 7.5 17.25 9.8505 17.25 12.75C17.25 15.6495 14.8995 18 12 18C9.1005 18 6.75 15.6495 6.75 12.75ZM18.75 11.25C19.1642 11.25 19.5 10.9142 19.5 10.5C19.5 10.0858 19.1642 9.75 18.75 9.75C18.3358 9.75 18 10.0858 18 10.5C18 10.9142 18.3358 11.25 18.75 11.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
