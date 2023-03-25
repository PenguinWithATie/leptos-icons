#[cfg(feature = "VsBellSlash")]
use leptos::*;
#[cfg(feature = "VsBellSlash")]
///This icon requires the feature `VsBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.0268 2.08559C10.2949 1.51028 9.41936 1.14252 8.48416 1.02673C7.79028 0.954953 7.08444 1.02673 6.41449 1.25404C5.74454 1.46938 5.13441 1.82828 4.61998 2.30682C4.10555 2.77339 3.68683 3.34764 3.41168 3.9817C3.13652 4.61576 2.98099 5.29767 2.98099 6.00351V8.20478C2.98099 8.97587 2.85921 9.74697 2.62628 10.4861L3.92646 9.18593C3.96573 8.86 3.98592 8.53239 3.98592 8.20478V6.00351C3.98592 5.44123 4.10555 4.89092 4.33286 4.38845C4.56016 3.87403 4.88318 3.41942 5.30189 3.04855C5.72061 2.66572 6.21111 2.3786 6.7375 2.21111C7.27586 2.03166 7.83814 1.97184 8.38846 2.03166C9.08536 2.12292 9.74775 2.39254 10.3078 2.80461L11.0268 2.08559ZM7.02379 12.0211H12.803L12.4321 10.9085C12.133 10.0352 11.9894 9.12596 11.9894 8.21674V7.05545L12.9938 6.05112C12.9946 6.099 12.9948 6.14694 12.9944 6.19493V8.19282C12.9944 9.00633 13.126 9.80788 13.3772 10.5735L13.9634 12.3441L13.4849 13.0021H9.97959C9.97959 13.5285 9.76425 14.0429 9.39338 14.4138C9.02252 14.7847 8.50809 15 7.9817 15C7.45531 15 6.94088 14.7847 6.57002 14.4138C6.21281 14.0566 5.99988 13.5662 5.98468 13.0602L7.02379 12.0211ZM7.9817 14.019C8.24489 14.019 8.49613 13.9113 8.68754 13.7199C8.87896 13.5285 8.98663 13.2773 8.97466 13.0141H6.97677C6.97677 13.2773 7.08444 13.5285 7.27586 13.7199C7.46727 13.9113 7.7185 14.019 7.9817 14.019Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "1" y = "14.5" width = "20"
        height = "1" transform = "rotate(-45 1 14.5)" /> < title > { title } < / title >
        < / svg >
    }
}
