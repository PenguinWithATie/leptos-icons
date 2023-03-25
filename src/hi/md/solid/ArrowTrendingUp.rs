#[cfg(feature = "HiMdSolidArrowTrendingUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowTrendingUp")]
///This icon requires the feature `HiMdSolidArrowTrendingUp` to be enabled.
#[component]
pub fn ArrowTrendingUp(
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
        "M12.577 4.87834C12.6842 4.47824 13.0955 4.2408 13.4956 4.34801L18.2766 5.6291C18.4688 5.68058 18.6326 5.80628 18.732 5.97854C18.8315 6.1508 18.8585 6.35552 18.807 6.54766L17.5259 11.3287C17.4187 11.7288 17.0074 11.9663 16.6073 11.8591C16.2072 11.7519 15.9698 11.3406 16.077 10.9405L16.8865 7.9195C14.6303 9.30965 12.7541 11.0901 11.2935 13.1222C11.1651 13.3009 10.9646 13.4142 10.7452 13.432C10.5259 13.4499 10.3098 13.3704 10.1542 13.2148L7 10.0607L2.28033 14.7803C1.98744 15.0732 1.51256 15.0732 1.21967 14.7803C0.926777 14.4874 0.926777 14.0126 1.21967 13.7197L6.46967 8.46968C6.76256 8.17679 7.23744 8.17679 7.53033 8.46968L10.6039 11.5433C12.1049 9.63051 13.9633 7.9506 16.1492 6.61197L13.1073 5.7969C12.7072 5.68969 12.4698 5.27844 12.577 4.87834Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
