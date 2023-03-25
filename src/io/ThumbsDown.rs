#[cfg(feature = "IoThumbsDown")]
use leptos::*;
#[cfg(feature = "IoThumbsDown")]
///This icon requires the feature `IoThumbsDown` to be enabled.
#[component]
pub fn ThumbsDown(
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
        "M39.94,178l144.16,6.12c4.61.36,23.9,1.22,23.9,25.88,0,23.8-19.16,25.33-24.14,25.88L39.94,242C27.27,241.87,16,227.56,16,210S27.27,178.13,39.94,178ZM181.39,309.66,74.65,318C62,318,48,301.31,48,284.12v-.33c0-16.33,11.14-29.63,24.88-29.79l108.45,1.72C208,259,208,275.16,208,282.12,208,305,186.2,309.26,181.39,309.66ZM90.15,32l89.37,8.93C204,41.86,208,58.18,208,68.4,208,86.79,194.59,93,181.33,93l-91,3C75.78,95.78,64,81.51,64,64S75.68,32.34,90.15,32ZM55.79,103.5l126.4,6.22c9.39.63,25.81,3,25.81,26.36,0,12-4.35,25.62-25,27.53L55.79,167.5C42.65,167.35,32,154,32,136.08S42.65,103.65,55.79,103.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M378.45,273.93A15.84,15.84,0,0,1,386,272h0a15.93,15.93,0,0,0-7.51,1.91Z" style =
        "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M337.86,343.22l-.13.22a2.53,2.53,0,0,1,.13-.22c20.5-35.51,30.36-55,33.82-62h0C368.21,288.28,358.34,307.73,337.86,343.22Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M372.66,279.16l-1,2a16.29,16.29,0,0,1,6.77-7.26A16.48,16.48,0,0,0,372.66,279.16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M195.94,459.38C205.37,472.67,221,480,240,480a16,16,0,0,0,14.31-8.85c3-6.06,15.25-24,28.19-42.9,18-26.33,40.35-59.08,55.23-84.81l.13-.22c20.48-35.49,30.35-54.94,33.82-62h0l1-2a16.48,16.48,0,0,1,5.79-5.23l0,0A15.93,15.93,0,0,1,386,272h25.32A84.7,84.7,0,0,0,496,187.3V148.7A84.7,84.7,0,0,0,411.31,64H362.52a17.46,17.46,0,0,1-9.58-2.89C330,46.13,286.66,32,240,32c-7.45,0-14.19.14-20.27.38a8,8,0,0,0-6.2,12.68l.1.14C222.2,57.59,224,71,224,80a61.16,61.16,0,0,1-5.19,24.77,17.38,17.38,0,0,0,0,14.06,63.81,63.81,0,0,1,0,50.39,17.32,17.32,0,0,0,0,14,62.13,62.13,0,0,1,0,49.58,18.13,18.13,0,0,0,0,14.68A60.41,60.41,0,0,1,224,273c0,8.2-2,21.3-8,31.18a15.66,15.66,0,0,0-1.14,13.65c.38,1,.76,2.06,1.13,3.17a24.8,24.8,0,0,1,.86,11.57c-3,19.35-9.67,36.3-16.74,54.16-3.08,7.78-6.27,15.82-9.22,24.27C184.75,428.56,186.59,446.2,195.94,459.38Z"
        /> < title > { title } < / title > < / svg >
    }
}
