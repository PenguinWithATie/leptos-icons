#[cfg(feature = "WiMeteor")]
use leptos::*;
#[cfg(feature = "WiMeteor")]
///This icon requires the feature `WiMeteor` to be enabled.
#[component]
pub fn Meteor(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M7.09,19.39c0-0.13,0-0.23,0.01-0.29v-0.08c0-0.02,0-0.04,0-0.06c0-0.02,0-0.03,0-0.05s0-0.03,0-0.05c0-0.02,0-0.03,0-0.04&#xA;	v-0.02c0-0.03,0.01-0.07,0.02-0.12c0.01-0.05,0.02-0.08,0.02-0.09v-0.03c0-0.01,0-0.03,0-0.05c0-0.02,0-0.03,0-0.04l0.08-0.37&#xA;	c0-0.01,0-0.01,0.01-0.02v-0.02l0.04-0.14c0.01-0.01,0.01-0.01,0.01-0.02c0.01-0.01,0.01-0.02,0.01-0.03v-0.03&#xA;	c0.04-0.12,0.07-0.22,0.1-0.28c0-0.01,0.01-0.02,0.02-0.03c0.01-0.01,0.02-0.06,0.05-0.15c0.17-0.38,0.38-0.74,0.63-1.08l0.06-0.07&#xA;	c0.01-0.01,0.02-0.02,0.03-0.04c0.01-0.02,0.02-0.03,0.03-0.04c0.01-0.01,0.03-0.03,0.07-0.06c0.01-0.02,0.02-0.04,0.04-0.06&#xA;	c0.02-0.02,0.03-0.04,0.04-0.06c0.04-0.02,0.06-0.05,0.07-0.07c0.01-0.01,0.03-0.02,0.07-0.06l0.07-0.07l7.6-8.33l-0.38,2.2&#xA;	l6.82-7.29l-4.18,8.14l4.18-3.16l-3.79,7.6l2.71-1.87l-4.68,8.33c0,0.01-0.01,0.02-0.02,0.04s-0.02,0.04-0.03,0.05&#xA;	c-0.01,0.01-0.01,0.02-0.02,0.04c-0.01,0.02-0.01,0.03-0.02,0.05c-0.01,0.01-0.01,0.02-0.02,0.05c-0.01,0.02-0.02,0.04-0.02,0.05&#xA;	c-0.43,0.84-1.05,1.51-1.86,2.02c-0.81,0.51-1.7,0.76-2.67,0.76c-0.92,0-1.77-0.23-2.55-0.68c-0.78-0.46-1.4-1.07-1.86-1.86&#xA;	S7.09,20.31,7.09,19.39z M8.29,19.39c0,1.08,0.38,1.99,1.14,2.75c0.76,0.76,1.68,1.14,2.75,1.14c0.82,0,1.56-0.24,2.22-0.71&#xA;	c0.66-0.47,1.13-1.09,1.41-1.84c0.17-0.43,0.25-0.87,0.25-1.34c0-1.07-0.38-1.99-1.13-2.75c-0.76-0.76-1.67-1.13-2.75-1.13&#xA;	c-1,0-1.87,0.33-2.6,1c-0.41,0.36-0.72,0.78-0.95,1.28C8.4,18.3,8.29,18.83,8.29,19.39z"
        /> < title > { title } < / title > < / svg >
    }
}
