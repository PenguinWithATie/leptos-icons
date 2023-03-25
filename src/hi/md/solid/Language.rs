#[cfg(feature = "HiMdSolidLanguage")]
use leptos::*;
#[cfg(feature = "HiMdSolidLanguage")]
///This icon requires the feature `HiMdSolidLanguage` to be enabled.
#[component]
pub fn Language(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.75009 2.75C7.75009 2.33579 7.4143 2 7.00009 2C6.58587 2 6.25009 2.33579 6.25009 2.75V4.00842C5.03323 4.03578 3.8325 4.12954 2.65142 4.28627C2.24081 4.34076 1.95211 4.7178 2.0066 5.12841C2.06109 5.53902 2.43813 5.82772 2.84875 5.77323C4.20658 5.59304 5.59228 5.5 7.00009 5.5C7.57065 5.5 8.13758 5.51528 8.70048 5.54546C8.2826 6.95686 7.70916 8.30168 6.99996 9.56001C6.62755 8.89925 6.29259 8.21466 5.99793 7.50912C5.8383 7.1269 5.39905 6.94646 5.01683 7.10608C4.63461 7.26571 4.45416 7.70497 4.61379 8.08718C5.03667 9.09975 5.5365 10.072 6.10593 10.9967C5.03656 12.5598 3.74539 13.9596 2.27753 15.1506C1.95588 15.4116 1.9067 15.8839 2.16769 16.2055C2.42867 16.5272 2.90099 16.5764 3.22264 16.3154C4.64811 15.1588 5.91919 13.8189 6.99996 12.3314C7.0954 12.4628 7.19233 12.593 7.29071 12.722C7.54187 13.0514 8.01249 13.1148 8.34187 12.8637C8.67125 12.6125 8.73467 12.1419 8.48351 11.8125C8.2802 11.5459 8.08361 11.2739 7.894 10.9967C8.90466 9.35554 9.69602 7.56455 10.227 5.66453C10.5365 5.69627 10.8447 5.73253 11.1514 5.77323C11.562 5.82772 11.9391 5.53902 11.9936 5.12841C12.0481 4.7178 11.7594 4.34076 11.3487 4.28627C10.8114 4.21496 10.27 4.15669 9.72489 4.11178C9.07179 4.05797 8.41333 4.02333 7.75009 4.00842V2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M13.0002 8C13.2842 8 13.5439 8.1605 13.671 8.41459L17.921 16.9146C18.1062 17.2851 17.9561 17.7356 17.5856 17.9208C17.2151 18.1061 16.7646 17.9559 16.5793 17.5854L15.7866 16H10.2137L9.42098 17.5854C9.23574 17.9559 8.78524 18.1061 8.41475 17.9208C8.04427 17.7356 7.8941 17.2851 8.07934 16.9146L12.3293 8.41459C12.4564 8.1605 12.7161 8 13.0002 8ZM15.0366 14.5L13.0002 10.4271L10.9637 14.5H15.0366Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
