#[cfg(feature = "HiLgOutlineCog8Tooth")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCog8Tooth")]
///This icon requires the feature `HiLgOutlineCog8Tooth` to be enabled.
#[component]
pub fn Cog8Tooth(
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
        "M10.3427 3.94005C10.4331 3.39759 10.9024 3 11.4523 3H12.5463C13.0963 3 13.5656 3.39759 13.656 3.94005L13.805 4.83386C13.8757 5.25813 14.1886 5.59838 14.5859 5.76332C14.9833 5.92832 15.4396 5.90629 15.7898 5.65617L16.5273 5.12933C16.9749 4.80969 17.5879 4.86042 17.9767 5.24929L18.7503 6.02284C19.1392 6.41171 19.1899 7.02472 18.8702 7.47223L18.3432 8.21007C18.0932 8.56012 18.0711 9.01633 18.2361 9.41363C18.401 9.81078 18.7411 10.1236 19.1653 10.1943L20.0593 10.3433C20.6017 10.4337 20.9993 10.9031 20.9993 11.453V12.547C20.9993 13.0969 20.6017 13.5663 20.0593 13.6567L19.1655 13.8056C18.7412 13.8764 18.4009 14.1893 18.236 14.5865C18.071 14.9839 18.0931 15.4403 18.3432 15.7904L18.8699 16.5278C19.1895 16.9753 19.1388 17.5884 18.7499 17.9772L17.9764 18.7508C17.5875 19.1396 16.9745 19.1904 16.527 18.8707L15.7894 18.3439C15.4393 18.0938 14.9831 18.0718 14.5857 18.2367C14.1886 18.4016 13.8757 18.7418 13.805 19.166L13.656 20.0599C13.5656 20.6024 13.0963 21 12.5463 21H11.4523C10.9024 21 10.4331 20.6024 10.3427 20.0599L10.1937 19.1661C10.123 18.7419 9.81005 18.4016 9.41282 18.2367C9.01541 18.0717 8.55908 18.0937 8.20893 18.3438L7.47131 18.8707C7.0238 19.1904 6.41079 19.1396 6.02192 18.7507L5.24837 17.9772C4.8595 17.5883 4.80877 16.9753 5.12842 16.5278L5.65545 15.79C5.90549 15.4399 5.92754 14.9837 5.76258 14.5864C5.5977 14.1892 5.25752 13.8764 4.83335 13.8057L3.93938 13.6567C3.39692 13.5663 2.99933 13.0969 2.99933 12.547V11.453C2.99933 10.9031 3.39692 10.4337 3.93938 10.3433L4.83319 10.1944C5.25746 10.1236 5.59771 9.81071 5.76265 9.41347C5.92766 9.01605 5.90562 8.5597 5.6555 8.20954L5.12881 7.47216C4.80916 7.02465 4.85989 6.41164 5.24876 6.02277L6.02231 5.24922C6.41118 4.86036 7.02419 4.80962 7.4717 5.12927L8.2093 5.65613C8.55937 5.90618 9.01561 5.92822 9.41293 5.76326C9.8101 5.59837 10.123 5.25819 10.1937 4.834L10.3427 3.94005Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 12C15 13.6569 13.6569 15 12 15C10.3432 15 9.00003 13.6569 9.00003 12C9.00003 10.3432 10.3432 9.00002 12 9.00002C13.6569 9.00002 15 10.3432 15 12Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
