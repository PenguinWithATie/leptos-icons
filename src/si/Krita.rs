#[cfg(feature = "SiKrita")]
use leptos::*;
#[cfg(feature = "SiKrita")]
///This icon requires the feature `SiKrita` to be enabled.
#[component]
pub fn Krita(
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
        "M.652.76a.625.625 0 00-.5.246c-.352.448-.035.898.362 1.262.206.189 1.77 1.794 3.428 3.527a11.054 11.054 0 011.815-1.983C3.667 2.515 1.694 1.266 1.461 1.1 1.201.914.917.762.652.76zm5.105 3.052c1.848 1.148 3.786 2.332 4.693 2.84 1.469.821 3.758 2.684 4.092 4.434.535.466 2.182 1.916 2.596 2.413.698-.211 1.518.133 2.06 1.12.866 1.583.227 3.747-1.968 4.988a5.42 5.42 0 01-.296.267l.296-.267c1.14-1.468-.714-2.44-1.175-3.864a2.06 2.06 0 01-.11-.78c-.533-.282-2.11-1.452-2.795-1.965-1.801.16-4.207-1.773-5.35-3.08-.7-.802-2.32-2.517-3.858-4.123a11.052 11.052 0 00-2.046 6.393A11.052 11.052 0 1012.948 1.136c-2.64.004-5.19.954-7.19 2.676zm8.71 7.552c-.515.126-.968.831-1.118 1.306-.038.115-.04.303.066.342.802.592 1.556 1.168 2.4 1.7.162-.393.746-.963 1.096-1.2zm-11.53 1.639c.812 1.898 5.798 7.17 12.06 2.695a2.07 2.07 0 00.114.715c.46 1.42 2.36 2.427 1.238 3.89-2.135 1.364-5 1.201-6.989.528-3.558-1.204-5.914-4.332-6.424-7.828zm13.782.7a.771.771 0 00-.065.049c-.004.003-.008.008-.011.008.003-.003.007-.008.01-.008.024-.015.044-.034.066-.048z"
        /> < title > { title } < / title > < / svg >
    }
}
