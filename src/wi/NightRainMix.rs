#[cfg(feature = "WiNightRainMix")]
use leptos::*;
#[cfg(feature = "WiNightRainMix")]
///This icon requires the feature `WiNightRainMix` to be enabled.
#[component]
pub fn NightRainMix(
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
        "M4.19,16.91c0,0.87,0.21,1.68,0.64,2.43c0.42,0.75,1.01,1.35,1.74,1.8C7.3,21.6,8.11,21.84,9,21.86&#xA;	c0.12,0,0.19-0.06,0.19-0.17v-1.34c0-0.12-0.06-0.18-0.19-0.18c-0.86-0.04-1.59-0.39-2.19-1.03s-0.91-1.39-0.91-2.24&#xA;	c0-0.85,0.28-1.59,0.85-2.21c0.57-0.62,1.27-0.97,2.11-1.04l0.52-0.07c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52&#xA;	c0.11-1.1,0.57-2.02,1.38-2.76s1.77-1.11,2.87-1.11c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.65,1.4,2.73l0.08,0.58&#xA;	c0,0.11,0.06,0.17,0.18,0.17h1.62c0.9,0,1.67,0.32,2.32,0.97c0.65,0.65,0.97,1.42,0.97,2.32c0,0.85-0.3,1.6-0.91,2.24&#xA;	c-0.61,0.64-1.33,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17c0.88-0.02,1.69-0.26,2.42-0.72&#xA;	c0.73-0.45,1.31-1.05,1.73-1.8s0.63-1.56,0.63-2.43c0-0.73-0.14-1.4-0.42-2.02c0.81-0.99,1.21-2.12,1.21-3.37&#xA;	c0-0.96-0.24-1.84-0.71-2.65s-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71c-0.74,0-1.46,0.15-2.15,0.46&#xA;	C17.71,7.03,17.12,7.45,16.62,8c-0.88-0.43-1.78-0.65-2.71-0.65c-1.42,0-2.68,0.44-3.78,1.32s-1.81,2-2.12,3.37&#xA;	c-1.12,0.29-2.04,0.88-2.76,1.78C4.54,14.72,4.19,15.75,4.19,16.91z M9.52,23.98c0,0.17,0.05,0.34,0.16,0.51&#xA;	c0.11,0.17,0.27,0.28,0.47,0.35c0.23,0.07,0.44,0.06,0.64-0.04c0.19-0.09,0.32-0.28,0.39-0.56l0.14-0.61&#xA;	c0.05-0.23,0.02-0.44-0.09-0.63c-0.11-0.2-0.28-0.33-0.52-0.4c-0.22-0.07-0.44-0.04-0.64,0.08s-0.34,0.3-0.4,0.53l-0.14,0.59&#xA;	C9.53,23.83,9.52,23.89,9.52,23.98z M10.28,21.08c0,0.21,0.08,0.4,0.25,0.57c0.16,0.17,0.34,0.25,0.56,0.25&#xA;	c0.24,0,0.44-0.08,0.6-0.24c0.16-0.16,0.24-0.35,0.24-0.59c0-0.23-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.24-0.6-0.24&#xA;	c-0.23,0-0.42,0.08-0.58,0.23C10.36,20.65,10.28,20.85,10.28,21.08z M10.89,18.81c-0.01,0.16,0.03,0.31,0.14,0.45&#xA;	c0.1,0.15,0.26,0.25,0.48,0.32c0.21,0.06,0.41,0.04,0.62-0.07c0.21-0.11,0.34-0.28,0.41-0.51l0.28-0.9&#xA;	c0.07-0.24,0.05-0.46-0.07-0.65c-0.12-0.19-0.3-0.32-0.54-0.39c-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.34,0.28-0.41,0.5&#xA;	l-0.24,0.92c0,0.02-0.01,0.06-0.02,0.12C10.9,18.72,10.89,18.77,10.89,18.81z M12.05,27.1c0,0.18,0.05,0.34,0.15,0.5&#xA;	c0.1,0.16,0.26,0.27,0.48,0.33c0.08,0.02,0.17,0.03,0.25,0.03c0.43,0,0.69-0.2,0.79-0.61l0.14-0.59c0.06-0.26,0.03-0.48-0.08-0.68&#xA;	s-0.29-0.32-0.52-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.14,0.59&#xA;	C12.06,26.97,12.05,27.04,12.05,27.1z M12.83,24.2c0,0.22,0.08,0.41,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24&#xA;	c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23&#xA;	s-0.43,0.08-0.59,0.23C12.91,23.77,12.83,23.97,12.83,24.2z M13.46,21.93c-0.01,0.15,0.03,0.31,0.13,0.47s0.25,0.26,0.45,0.3&#xA;	c0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.27-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4&#xA;	c-0.22-0.07-0.43-0.04-0.64,0.08c-0.21,0.12-0.34,0.3-0.41,0.53l-0.23,0.9C13.47,21.74,13.46,21.83,13.46,21.93z M16.21,24.08&#xA;	c0,0.16,0.05,0.32,0.15,0.48s0.26,0.27,0.46,0.33c0.03,0,0.08,0.01,0.14,0.02s0.1,0.02,0.14,0.02c0.41,0,0.66-0.22,0.76-0.66&#xA;	l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.11-0.21-0.28-0.34-0.51-0.41c-0.25-0.06-0.48-0.04-0.68,0.08&#xA;	c-0.2,0.12-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.02-0.01,0.07-0.02,0.12S16.21,24.04,16.21,24.08z M16.95,21.12&#xA;	c0,0.22,0.08,0.42,0.25,0.57c0.15,0.16,0.34,0.24,0.57,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58&#xA;	c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23S16.95,20.88,16.95,21.12z M17.56,18.81&#xA;	c0,0.17,0.05,0.33,0.16,0.48s0.27,0.26,0.49,0.32c0.02,0,0.06,0.01,0.12,0.02s0.11,0.02,0.14,0.02c0.1,0,0.22-0.03,0.36-0.09&#xA;	c0.21-0.11,0.35-0.29,0.41-0.52l0.24-0.9c0.06-0.23,0.04-0.44-0.07-0.63c-0.11-0.2-0.28-0.33-0.51-0.4&#xA;	c-0.23-0.07-0.44-0.05-0.64,0.06c-0.19,0.11-0.32,0.27-0.39,0.51l-0.28,0.91c0,0.02-0.01,0.06-0.02,0.12&#xA;	C17.57,18.74,17.56,18.78,17.56,18.81z M18.03,9.01c0.69-0.69,1.53-1.04,2.51-1.04c0.98,0,1.82,0.35,2.51,1.05&#xA;	c0.69,0.7,1.04,1.54,1.04,2.52c0,0.67-0.17,1.28-0.51,1.85c-0.96-0.96-2.14-1.44-3.54-1.44h-0.32C19.44,10.77,18.88,9.79,18.03,9.01&#xA;	z"
        /> < title > { title } < / title > < / svg >
    }
}
