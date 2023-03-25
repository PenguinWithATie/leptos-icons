#[cfg(feature = "HiLgSolidChatBubbleLeftRight")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleLeftRight")]
///This icon requires the feature `HiLgSolidChatBubbleLeftRight` to be enabled.
#[component]
pub fn ChatBubbleLeftRight(
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
        "M4.91307 2.65823C6.9877 2.38888 9.10296 2.25 11.2503 2.25C13.3974 2.25 15.5124 2.38885 17.5869 2.65815C19.5091 2.90769 20.8783 4.51937 20.9923 6.38495C20.6665 6.27614 20.3212 6.20396 19.96 6.17399C18.5715 6.05874 17.1673 6 15.75 6C14.3326 6 12.9285 6.05874 11.54 6.17398C9.1817 6.36971 7.5 8.36467 7.5 10.6082V14.8937C7.5 16.5844 8.45468 18.1326 9.9328 18.8779L7.28033 21.5303C7.06583 21.7448 6.74324 21.809 6.46299 21.6929C6.18273 21.5768 6 21.3033 6 21V16.9705C5.63649 16.9316 5.27417 16.8887 4.91308 16.8418C2.90466 16.581 1.5 14.8333 1.5 12.8626V6.63738C1.5 4.66672 2.90466 2.91899 4.91307 2.65823Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 7.5C14.3741 7.5 13.0114 7.55702 11.6641 7.66884C10.1248 7.7966 9 9.10282 9 10.6082V14.8937C9 16.4014 10.128 17.7083 11.6692 17.8341C12.9131 17.9357 14.17 17.9912 15.4384 17.999L18.2197 20.7803C18.4342 20.9948 18.7568 21.059 19.037 20.9429C19.3173 20.8268 19.5 20.5533 19.5 20.25V17.8601C19.6103 17.8518 19.7206 17.8432 19.8307 17.8342C21.372 17.7085 22.5 16.4015 22.5 14.8938V10.6082C22.5 9.10283 21.3752 7.79661 19.836 7.66885C18.4886 7.55702 17.1259 7.5 15.75 7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
