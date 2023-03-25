#[cfg(feature = "HiLgOutlineGlobeEuropeAfrica")]
use leptos::*;
#[cfg(feature = "HiLgOutlineGlobeEuropeAfrica")]
///This icon requires the feature `HiLgOutlineGlobeEuropeAfrica` to be enabled.
#[component]
pub fn GlobeEuropeAfrica(
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
        "M20.8929 13.3929L19.7582 12.2582C19.5872 12.0872 19.4449 11.8897 19.3367 11.6734L18.2567 9.5133C18.1304 9.26078 17.7938 9.20616 17.5942 9.4058C17.3818 9.61824 17.0708 9.69881 16.782 9.61627L15.5091 9.25259C15.0257 9.11447 14.5239 9.40424 14.402 9.892C14.3108 10.2566 14.4587 10.6392 14.7715 10.8476L15.3582 11.2388C15.9489 11.6326 16.0316 12.4684 15.5297 12.9703L15.3295 13.1705C15.1185 13.3815 15 13.6676 15 13.966V14.3768C15 14.7846 14.8892 15.1847 14.6794 15.5344L13.3647 17.7254C12.9834 18.3611 12.2964 18.75 11.5552 18.75C10.9724 18.75 10.5 18.2776 10.5 17.6948V16.5233C10.5 15.6033 9.93986 14.7759 9.08563 14.4343L8.43149 14.1726C7.44975 13.7799 6.8739 12.7566 7.04773 11.7136L7.05477 11.6714C7.10117 11.393 7.19956 11.1257 7.3448 10.8837L7.43421 10.7347C7.92343 9.91928 8.87241 9.49948 9.80483 9.68597L10.9827 9.92153C11.5574 10.0365 12.124 9.69096 12.285 9.12744L12.4935 8.39774C12.6422 7.87721 12.3991 7.32456 11.9149 7.08245L11.25 6.75L11.159 6.84099C10.7371 7.26295 10.1648 7.5 9.56802 7.5H9.38709C9.13924 7.5 8.90095 7.59905 8.7257 7.7743C8.44222 8.05778 8.00814 8.12907 7.64958 7.94979C7.16433 7.70716 6.98833 7.10278 7.26746 6.63757L8.67936 4.2844C8.82024 4.04961 8.91649 3.79207 8.96453 3.52474M20.8929 13.3929C20.9634 12.9389 21 12.4737 21 12C21 7.02944 16.9706 3 12 3C10.9348 3 9.91287 3.18504 8.96453 3.52474M20.8929 13.3929C20.2234 17.702 16.4968 21 12 21C7.02944 21 3 16.9706 3 12C3 8.09461 5.48749 4.77021 8.96453 3.52474"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
