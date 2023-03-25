#[cfg(feature = "HiLgSolidGlobeAsiaAustralia")]
use leptos::*;
#[cfg(feature = "HiLgSolidGlobeAsiaAustralia")]
///This icon requires the feature `HiLgSolidGlobeAsiaAustralia` to be enabled.
#[component]
pub fn GlobeAsiaAustralia(
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
        "M15.7498 8.25C16.164 8.25 16.4998 8.58579 16.4998 9C16.4998 10.1201 16.0077 11.1263 15.2304 11.8123C14.9199 12.0864 14.4459 12.0569 14.1718 11.7463C13.8977 11.4357 13.9273 10.9618 14.2378 10.6877C14.7062 10.2743 14.9998 9.67191 14.9998 9C14.9998 8.58579 15.3356 8.25 15.7498 8.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M12 2.25C6.61522 2.25 2.25 6.61522 2.25 12C2.25 17.3848 6.61522 21.75 12 21.75C17.3848 21.75 21.75 17.3848 21.75 12C21.75 6.61522 17.3848 2.25 12 2.25ZM4.5749 15.6002C5.91195 18.3527 8.7344 20.25 12 20.25C12.6623 20.25 13.3063 20.172 13.9234 20.0246C13.6364 19.2603 12.9044 18.75 12.0828 18.75C11.6022 18.75 11.1921 18.4026 11.1131 17.9285L11.0401 17.4907C10.9461 16.9263 11.2906 16.3813 11.8408 16.2241L12.8295 15.9416C13.2567 15.8195 13.6131 15.5237 13.8119 15.1263L13.8484 15.0533C14.0945 14.561 14.5977 14.25 15.1481 14.25C15.5335 14.25 15.9031 14.4031 16.1756 14.6756L16.5 15H17.1283C17.9669 15 18.724 15.4646 19.1108 16.1857C19.8347 14.9586 20.25 13.5278 20.25 12C20.25 7.7018 16.963 4.17132 12.7656 3.78506C12.81 4.04802 12.9468 4.289 13.1548 4.46233L14.2234 5.35284C14.6651 5.7209 14.7582 6.36275 14.4393 6.84112L13.9282 7.60766C13.6507 8.02398 13.2423 8.3359 12.7676 8.49413L12.6254 8.54154C11.9327 8.77243 11.6492 9.59877 12.0542 10.2063C12.4237 10.7605 12.2238 11.5131 11.6281 11.811L9 13.125L9.42339 14.1835C9.608 14.645 9.40803 15.171 8.96343 15.3933C8.5503 15.5999 8.04855 15.4814 7.77142 15.1119L7.09217 14.2062C6.59039 13.5372 5.55995 13.6301 5.18594 14.3781L4.5749 15.6002Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
