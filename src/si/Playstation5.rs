#[cfg(feature = "SiPlaystation5")]
use leptos::*;
#[cfg(feature = "SiPlaystation5")]
///This icon requires the feature `SiPlaystation5` to be enabled.
#[component]
pub fn Playstation5(
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
        "M10.4499 14.56905a1.38287 1.38287 0 001.38287-1.38287v-2.37841a.83315.83315 0 01.83416-.83315h2.68403a.03732.03732 0 00.03631-.03732V9.4612a.03631.03631 0 00-.0363-.0363H12.1172a1.38287 1.38287 0 00-1.38388 1.38286v2.38043a.83416.83416 0 01-.83315.83415H7.25347a.03631.03631 0 00-.03631.03632v.47608a.03631.03631 0 00.03631.03631zm6.04488-3.21156V9.4612a.03631.03631 0 01.0363-.0363h7.30772a.03732.03732 0 01.03732.0363v.47609a.03833.03833 0 01-.03732.03732h-6.20929a.03631.03631 0 00-.0363.03631v1.2356a.3954.3954 0 00.3964.39741h4.62267a1.46457 1.46457 0 010 2.9251h-6.0812a.03631.03631 0 01-.0363-.0363v-.47407a.03631.03631 0 01.0363-.03632h5.53047a.91586.91586 0 10-.00706-1.8307h-4.72656a.83315.83315 0 01-.83315-.83416m-10.84608.28645a.83466.83466 0 000-1.66932H.03654a.03732.03732 0 01-.03632-.03732V9.4612a.03631.03631 0 01.03632-.0363h6.1528a1.38388 1.38388 0 010 2.76673H1.9328a.83315.83315 0 00-.83315.83416v1.51299a.03631.03631 0 01-.03631.0363H.03654a.03631.03631 0 01-.03632-.04034v-1.51298a1.38287 1.38287 0 011.38388-1.37783Z"
        /> < title > { title } < / title > < / svg >
    }
}
