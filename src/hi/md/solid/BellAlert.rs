#[cfg(feature = "HiMdSolidBellAlert")]
use leptos::*;
#[cfg(feature = "HiMdSolidBellAlert")]
///This icon requires the feature `HiMdSolidBellAlert` to be enabled.
#[component]
pub fn BellAlert(
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
        "M4.21444 3.2267C4.47824 2.90735 4.43321 2.43462 4.11386 2.17082C3.79452 1.90702 3.32178 1.95204 3.05798 2.27139C2.1587 3.36002 1.50992 4.66481 1.20153 6.09691C1.11433 6.50184 1.37191 6.90079 1.77684 6.98799C2.18177 7.07519 2.58072 6.81762 2.66792 6.41268C2.92443 5.22148 3.46427 4.13482 4.21444 3.2267Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.9417 2.27139C16.6779 1.95204 16.2051 1.90702 15.8858 2.17082C15.5664 2.43462 15.5214 2.90735 15.7852 3.2267C16.5354 4.13482 17.0752 5.22148 17.3317 6.41268C17.4189 6.81762 17.8179 7.07519 18.2228 6.98799C18.6277 6.90079 18.8853 6.50184 18.7981 6.09691C18.4897 4.66481 17.8409 3.36002 16.9417 2.27139Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9.99997 1.99988C6.68626 1.99988 3.99997 4.68617 3.99997 7.99988C3.99997 9.88651 3.54624 11.665 2.7426 13.2342C2.63591 13.4425 2.6326 13.6887 2.73365 13.8998C2.83469 14.111 3.02851 14.2628 3.25769 14.3104C4.32537 14.5321 5.41181 14.7021 6.51426 14.8179C6.67494 16.6019 8.17421 17.9999 10 17.9999C11.8258 17.9999 13.3251 16.6019 13.4857 14.8179C14.5882 14.7021 15.6746 14.5321 16.7422 14.3104C16.9714 14.2628 17.1652 14.111 17.2663 13.8998C17.3673 13.6887 17.364 13.4425 17.2573 13.2342C16.4537 11.665 16 9.88651 16 7.99988C16 4.68617 13.3137 1.99988 9.99997 1.99988ZM10 16.4999C9.04777 16.4999 8.25097 15.8344 8.0493 14.9432C8.69477 14.9808 9.34517 14.9999 9.99997 14.9999C10.6548 14.9999 11.3052 14.9808 11.9507 14.9432C11.749 15.8344 10.9522 16.4999 10 16.4999Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
