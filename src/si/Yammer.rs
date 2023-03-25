#[cfg(feature = "SiYammer")]
use leptos::*;
#[cfg(feature = "SiYammer")]
///This icon requires the feature `SiYammer` to be enabled.
#[component]
pub fn Yammer(
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
        "M23.5094 7.391a.696.696 0 00-.859-.527l-2.31.626A17.4135 17.4135 0 0016.3897.226a.69.69 0 00-.509-.225.677.677 0 00-.482.2L9.7667 5.8379H1.023C.458 5.838 0 6.296 0 6.862v10.2368c0 .566.458 1.025 1.023 1.025h8.7037l5.6719 5.6768a.687.687 0 00.99-.025 17.4305 17.4305 0 003.9509-7.2638l2.3109.626a.696.696 0 00.859-.527 21.9024 21.9024 0 000-9.2198zm-7.6738-5.45a15.8536 15.8536 0 013.0229 5.9499l-6.5958 1.786v-2.815a1.02 1.02 0 00-.48-.865zM9.2738 9.226l-2.205 3.8809v2.0219a.938.938 0 11-1.876 0v-2.193L3.085 9.226a.8637.8637 0 111.501-.855l1.594 2.9779 1.5939-2.978a.861.861 0 011.176-.324.866.866 0 01.324 1.179zm9.5847 6.8848a15.8536 15.8536 0 01-3.023 5.9498l-4.0788-4.0819c.301-.178.506-.504.506-.88v-2.7739zm3.316-.698l-9.9118-2.684v-1.4559l9.9117-2.684a20.4075 20.4075 0 010 6.8239Z"
        /> < title > { title } < / title > < / svg >
    }
}
