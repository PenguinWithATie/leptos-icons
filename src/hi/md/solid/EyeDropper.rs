#[cfg(feature = "HiMdSolidEyeDropper")]
use leptos::*;
#[cfg(feature = "HiMdSolidEyeDropper")]
///This icon requires the feature `HiMdSolidEyeDropper` to be enabled.
#[component]
pub fn EyeDropper(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.0998 3.66668C12.4737 2.136 13.8542 1 15.5 1C15.5146 0.999999 15.5292 1.00009 15.5438 1.00027C15.9323 1.00503 16.3056 1.07311 16.6538 1.19465C17.1364 1.3626 17.5896 1.6393 17.9754 2.02503C18.892 2.94167 19.193 4.23984 18.8818 5.40523C18.7282 5.98079 18.4253 6.52479 17.9754 6.97477C17.6708 7.27935 17.3231 7.51652 16.9518 7.68564C16.7625 7.77206 16.5641 7.8421 16.3585 7.89394L16 7.99V9.99998C16 10.1989 15.921 10.3897 15.7803 10.5303L15.5303 10.7803C15.2374 11.0732 14.7626 11.0732 14.4697 10.7803L13.625 9.93564L7.21967 16.341C6.79771 16.7629 6.22541 17 5.62868 17H5.12132C4.92241 17 4.73164 17.079 4.59099 17.2196L3.03033 18.7803C2.73744 19.0732 2.26256 19.0732 1.96967 18.7803L1.21967 18.0303C0.926777 17.7374 0.926777 17.2625 1.21967 16.9696L2.78033 15.409C2.92098 15.2683 3 15.0776 3 14.8787V14.3713C3 13.7746 3.23705 13.2023 3.65901 12.7803L10.0643 6.37498L9.21967 5.53031C8.92678 5.23741 8.92678 4.76254 9.21967 4.46965L9.46967 4.21965C9.61032 4.07899 9.80109 3.99998 10 3.99998H12.0105L12.0998 3.66668ZM4.71967 13.841L11.125 7.43564L12.5643 8.87498L6.15901 15.2803C6.01836 15.421 5.82759 15.5 5.62868 15.5H5.12132C4.86336 15.5 4.60997 15.5443 4.37132 15.6287C4.4557 15.39 4.5 15.1366 4.5 14.8787V14.3713C4.5 14.1724 4.57902 13.9816 4.71967 13.841Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
