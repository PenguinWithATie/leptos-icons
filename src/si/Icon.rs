#[cfg(feature = "SiIcon")]
use leptos::*;
#[cfg(feature = "SiIcon")]
///This icon requires the feature `SiIcon` to be enabled.
#[component]
pub fn Icon(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1.92669 23.93457c-.93754-.17758-1.70436-.94464-1.88217-1.88241-.31993-1.6878 1.13237-3.1401 2.82018-2.82018.93754.17781 1.7046.94463 1.8824 1.88217.31993 1.68804-1.13237 3.14034-2.82041 2.82042zM21.13507 4.76808c-.93754-.1778-1.7046-.94463-1.8824-1.8824-.31993-1.68805 1.13284-3.14034 2.82065-2.82019.93777.17805 1.70436.94487 1.88217 1.88241.31992 1.6878-1.13261 3.1401-2.82042 2.82018zm-9.11415 1.24226c1.1475 0 2.21912.32347 3.13017.88292l2.58538-2.58562c-1.59582-1.1877-3.57352-1.89092-5.71555-1.89092-5.29278 0-9.58347 4.29045-9.58347 9.58323 0 2.14227.70321 4.11997 1.89116 5.7158l2.58538-2.5854c-.55945-.91105-.88268-1.9829-.88268-3.1304 0-3.30799 2.68162-5.98961 5.9896-5.98961zm5.10664 2.85936c.55969.91106.88292 1.98267.88292 3.13018 0 3.30798-2.68162 5.9896-5.98961 5.9896-1.1475 0-2.21935-.323-3.13041-.88268L6.30508 19.6922c1.59582 1.18794 3.57352 1.89115 5.71579 1.89115 5.29278 0 9.58323-4.29045 9.58323-9.58346 0-2.14227-.70345-4.11974-1.89092-5.7158Z"
        /> < title > { title } < / title > < / svg >
    }
}
