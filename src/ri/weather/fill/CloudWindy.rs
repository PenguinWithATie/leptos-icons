#[cfg(feature = "RiWeatherFillCloudWindy")]
use leptos::*;
#[cfg(feature = "RiWeatherFillCloudWindy")]
///This icon requires the feature `RiWeatherFillCloudWindy` to be enabled.
#[component]
pub fn CloudWindy(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M14 18v-3.993H2.074a8 8 0 0 1 14.383-6.908A5.5 5.5 0 1 1 17.5 18h-3.499zm-8 2h10v2H6v-2zm-4-4h10v2H2v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
