#[cfg(feature = "HiLgSolidVariable")]
use leptos::*;
#[cfg(feature = "HiLgSolidVariable")]
///This icon requires the feature `HiLgSolidVariable` to be enabled.
#[component]
pub fn Variable(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M19.2535 2.29164C19.6447 2.15548 20.0722 2.36223 20.2083 2.75343C21.2033 5.61217 21.75 8.73352 21.75 12C21.75 15.2664 21.2033 18.3877 20.2083 21.2465C20.0722 21.6377 19.6447 21.8444 19.2535 21.7083C18.8623 21.5721 18.6555 21.1446 18.7917 20.7534C19.7313 18.0536 20.25 15.0989 20.25 12C20.25 8.901 19.7313 5.94634 18.7917 3.24649C18.6555 2.85529 18.8623 2.42779 19.2535 2.29164ZM5.02596 2.30476C5.40991 2.46018 5.59517 2.89742 5.43975 3.28137C4.3503 5.97278 3.75 8.91534 3.75 12C3.75 15.0846 4.3503 18.0271 5.43975 20.7185C5.59517 21.1025 5.40991 21.5397 5.02596 21.6952C4.64201 21.8506 4.20477 21.6653 4.04935 21.2814C2.88874 18.4142 2.25 15.2807 2.25 12C2.25 8.71918 2.88874 5.58573 4.04935 2.71855C4.20477 2.3346 4.64201 2.14934 5.02596 2.30476ZM9.30143 7.35691C10.1246 6.85034 11.205 7.24311 11.5106 8.16005L12.2266 10.308L13.601 8.24636C14.2504 7.27224 15.5007 6.89742 16.5789 7.35359L16.7922 7.44385C17.1737 7.60524 17.3521 8.04533 17.1907 8.4268C17.0293 8.80828 16.5892 8.98669 16.2078 8.8253L15.9944 8.73504C15.5797 8.55959 15.0989 8.70375 14.8491 9.07841L12.8275 12.1108L13.9124 15.3655L15.3569 14.4766C15.7097 14.2595 16.1717 14.3695 16.3887 14.7223C16.6058 15.075 16.4958 15.537 16.1431 15.7541L14.6986 16.643C13.8754 17.1496 12.795 16.7568 12.4894 15.8399L11.7734 13.6919L10.399 15.7536C9.74958 16.7277 8.49934 17.1025 7.42112 16.6463L7.20777 16.5561C6.82629 16.3947 6.64788 15.9546 6.80927 15.5731C6.97067 15.1916 7.41075 15.0132 7.79223 15.1746L8.00558 15.2649C8.42028 15.4403 8.90114 15.2962 9.15091 14.9215L11.1725 11.8891L10.0876 8.63439L8.64307 9.52332C8.2903 9.74041 7.82834 9.63042 7.61126 9.27765C7.39417 8.92488 7.50416 8.46292 7.85693 8.24583L9.30143 7.35691Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
