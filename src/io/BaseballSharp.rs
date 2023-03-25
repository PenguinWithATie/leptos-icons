#[cfg(feature = "IoBaseballSharp")]
use leptos::*;
#[cfg(feature = "IoBaseballSharp")]
///This icon requires the feature `IoBaseballSharp` to be enabled.
#[component]
pub fn BaseballSharp(
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
        "M302.16,56.2a13.88,13.88,0,0,1-3.42,8.91l-.11,5.13-27.71-.57c0,1.3.09,2.61.16,3.91h0a177.33,177.33,0,0,0,3.45,26.31l24.72-7.18,7.81,26.88-24.71,7.18a177.21,177.21,0,0,0,13.34,27.69l21.27-15,16.16,22.86-21.29,15.05q5.25,6.33,11.11,12.19c3.91,3.91,8,7.6,12.2,11.1l15.71-22.22,22.86,16.16L358,216.75a178.78,178.78,0,0,0,27.68,13.32l7.49-25.8,26.89,7.81-7.5,25.83a177.8,177.8,0,0,0,31,3.67l-.41-24.67,17.41-.29A208,208,0,0,0,301.82,52.93,13.65,13.65,0,0,1,302.16,56.2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M214,441.64l27.91.49a177.46,177.46,0,0,0-3.62-29.95l-24.14,7-7.81-26.88,24.11-7a177.92,177.92,0,0,0-13.33-27.68L196.79,372l-16.16-22.86L201,334.73c-3.49-4.22-7.19-8.3-11.09-12.2s-8-7.62-12.19-11.12l-13.91,19.68L141,314.93l13.9-19.66a177.26,177.26,0,0,0-27.7-13.33l-6.37,21.94-26.89-7.81,6.38-22A177.32,177.32,0,0,0,74,270.67c-1.59-.09-3.18-.16-4.78-.2l.4,22.34-17.71.32A207.88,207.88,0,0,0,213.72,459.61Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M444.65,302.67l-.55-33.06a206,206,0,0,1-39.33-4.74L397,291.8,370.06,284l7.83-27a206.91,206.91,0,0,1-36.06-17.35l-16.36,23.15-22.86-16.16,16.33-23.11a204.21,204.21,0,0,1-30-30L266.75,209.2l-16.16-22.87,22.17-15.67a206,206,0,0,1-17.38-36.06l-25.75,7.48-7.81-26.89,25.73-7.47q-2-9.21-3.18-18.64l-.47,0-.78-14h0l-.33-6-17.94-.32a13.38,13.38,0,0,1-1.79-.16l-6.35-.13.06-2.47a14,14,0,0,1-5.66-11.49,13.27,13.27,0,0,1,.13-1.67A208,208,0,0,0,52.16,217.43l16.1-.28.45,25.18,6.83.38,14,.77,0,.48q9.42,1.17,18.64,3.18l6.68-23L141.7,232,135,255a205.3,205.3,0,0,1,36.06,17.38l14.53-20.56L208.47,268,194,288.5a203.5,203.5,0,0,1,30,30l21.3-15,16.16,22.86L240.1,341.41a206.86,206.86,0,0,1,17.34,36.06l25.27-7.33L290.52,397l-25.24,7.33A205.9,205.9,0,0,1,270,442.63l29.42.53-.29,16.48a207.94,207.94,0,0,0,160-157.21Z"
        /> < title > { title } < / title > < / svg >
    }
}
