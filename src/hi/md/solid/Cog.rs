#[cfg(feature = "HiMdSolidCog")]
use leptos::*;
#[cfg(feature = "HiMdSolidCog")]
///This icon requires the feature `HiMdSolidCog` to be enabled.
#[component]
pub fn Cog(
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
        "M13.0242 9.24999C13.4944 9.24999 13.8513 8.81719 13.6614 8.38695C13.0412 6.9812 11.6352 6 10 6C9.85376 6 9.70936 6.00785 9.56719 6.02314C9.09929 6.07349 8.90249 6.59904 9.13779 7.00659L10.2165 8.87499C10.3505 9.10704 10.5981 9.24999 10.866 9.24999L13.0242 9.24999Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.83948 7.75785C7.60411 7.35018 7.05027 7.25778 6.77194 7.63742C6.28661 8.29942 6 9.11624 6 10C6 10.8838 6.28662 11.7006 6.77198 12.3626C7.05031 12.7423 7.60415 12.6499 7.83952 12.2422L8.91751 10.3751C9.05149 10.143 9.05149 9.85711 8.91751 9.62506L7.83948 7.75785Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.13785 12.9934C8.90255 13.401 9.09936 13.9265 9.56726 13.9769C9.70941 13.9922 9.85379 14 10 14C11.6352 14 13.0412 13.0188 13.6614 11.613C13.8513 11.1828 13.4944 10.75 13.0242 10.75H10.8661C10.5982 10.75 10.3506 10.8929 10.2166 11.125L9.13785 12.9934Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M14.1295 4.34724L14.7744 3.23028C14.9815 2.87156 14.8586 2.41286 14.4999 2.20576C14.1412 1.99865 13.6825 2.12156 13.4754 2.48028L12.8311 3.59615C12.1832 3.30927 11.4835 3.11784 10.75 3.03971V1.75C10.75 1.33579 10.4142 1 10 1C9.58579 1 9.25 1.33579 9.25 1.75V3.03971C8.51649 3.11784 7.81683 3.30927 7.16886 3.59616L6.52462 2.4803C6.31752 2.12158 5.85882 1.99867 5.5001 2.20578C5.14139 2.41289 5.01848 2.87158 5.22559 3.2303L5.87046 4.34725C5.28784 4.7736 4.77359 5.28784 4.34725 5.87046L3.23009 5.22547C2.87137 5.01836 2.41267 5.14127 2.20557 5.49999C1.99846 5.85871 2.12137 6.3174 2.48009 6.52451L3.59615 7.16887C3.30927 7.81683 3.11784 8.51649 3.03971 9.25H1.75C1.33579 9.25 1 9.58579 1 10C1 10.4142 1.33579 10.75 1.75 10.75H3.03971C3.11784 11.4835 3.30926 12.1832 3.59614 12.8311L2.48009 13.4755C2.12137 13.6826 1.99846 14.1413 2.20557 14.5C2.41267 14.8587 2.87137 14.9816 3.23009 14.7745L4.34723 14.1295C4.77355 14.7121 5.28775 15.2263 5.87031 15.6526L5.22538 16.7697C5.01827 17.1284 5.14118 17.5871 5.4999 17.7942C5.85861 18.0013 6.31731 17.8784 6.52441 17.5197L7.1687 16.4038C7.81671 16.6907 8.51643 16.8822 9.25 16.9603V18.25C9.25 18.6642 9.58579 19 10 19C10.4142 19 10.75 18.6642 10.75 18.25V16.9603C11.4836 16.8822 12.1833 16.6907 12.8313 16.4038L13.4756 17.5197C13.6827 17.8784 14.1414 18.0013 14.5001 17.7942C14.8588 17.5871 14.9817 17.1284 14.7746 16.7697L14.1297 15.6526C14.7122 15.2263 15.2264 14.7121 15.6527 14.1296L16.7698 14.7745C17.1285 14.9816 17.5872 14.8587 17.7943 14.5C18.0014 14.1413 17.8785 13.6826 17.5198 13.4755L16.4038 12.8312C16.6907 12.1832 16.8822 11.4835 16.9603 10.75H18.25C18.6642 10.75 19 10.4142 19 10C19 9.58579 18.6642 9.25 18.25 9.25H16.9603C16.8822 8.51646 16.6907 7.81678 16.4038 7.16879L17.5198 6.52451C17.8785 6.3174 18.0014 5.85871 17.7943 5.49999C17.5872 5.14127 17.1285 5.01836 16.7698 5.22547L15.6527 5.8704C15.2264 5.2878 14.7121 4.77358 14.1295 4.34724ZM10 4.5C9.0112 4.5 8.08334 4.76094 7.28153 5.2177C7.27126 5.22431 7.26079 5.2307 7.2501 5.23687C7.23978 5.24283 7.22937 5.24852 7.21889 5.25393C6.40668 5.7309 5.72776 6.4104 5.25148 7.22307C5.24674 7.2321 5.2418 7.24107 5.23666 7.24999C5.2313 7.25926 5.22578 7.26837 5.2201 7.27733C4.76185 8.0801 4.5 9.00947 4.5 10C4.5 10.9904 4.76179 11.9197 5.21995 12.7224C5.22569 12.7314 5.23126 12.7406 5.23666 12.75C5.24185 12.759 5.24683 12.768 5.25161 12.7772C5.72819 13.5903 6.40765 14.27 7.2205 14.747C7.23036 14.7521 7.24017 14.7575 7.2499 14.7631C7.26 14.769 7.26992 14.775 7.27965 14.7812C8.08189 15.2387 9.01042 15.5 10 15.5C10.9897 15.5 11.9184 15.2386 12.7207 14.781C12.7303 14.7749 12.7401 14.7689 12.7501 14.7632C12.7597 14.7576 12.7694 14.7523 12.7792 14.7472C13.5913 14.2707 14.2704 13.5918 14.7469 12.7797C14.7521 12.7697 14.7575 12.7598 14.7632 12.75C14.7691 12.7398 14.7751 12.7298 14.7814 12.72C15.2387 11.9179 15.5 10.9894 15.5 10C15.5 9.01046 15.2387 8.08196 14.7813 7.27974C14.7751 7.27001 14.769 7.26009 14.7632 7.24999C14.7576 7.24025 14.7522 7.23044 14.7471 7.22057C14.2708 6.40891 13.5923 5.73024 12.7808 5.25375C12.7704 5.24838 12.7601 5.24275 12.7499 5.23685C12.7393 5.23074 12.7289 5.22441 12.7188 5.21788C11.9169 4.761 10.9889 4.5 10 4.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
