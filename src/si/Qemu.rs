#[cfg(feature = "SiQemu")]
use leptos::*;
#[cfg(feature = "SiQemu")]
///This icon requires the feature `SiQemu` to be enabled.
#[component]
pub fn Qemu(
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
        "M12.003.064C5.376.064 0 5.407 0 12s5.376 11.936 12.003 11.936c2.169 0 4.2-.57 5.955-1.57l.624 1.57h4.841l-1.893-4.679A11.845 11.845 0 0024 12C24 5.407 18.63.064 12.003.064zM8.818 2.03c.398.339.324.198.86.134.61-.397.893.942 1.147.195.748.097 1.542.34 2.25.584a3.447 3.447 0 011.859 1.128l-.014.007.35.463c.045.08.082.164.12.248.142 1.205 1.48 1.19 2.377 1.625.767.272 1.69.686 1.785 1.611-.193-.042-.941-.921-1.53-1.007a3.919 3.919 0 01-1.094-.255L14.86 6.38v-.007a3.035 3.035 0 01-.309-.053v.013l-2.927-.362c.048.033.1.077.148.12l3 .585v-.007l.209.053.839.188c.166.016.334.043.47.067.856.236 1.868.194 2.571.792-.184.352-1.21.153-1.719.108-.062-.012-.131-.023-.194-.034l-.034-.007c-.696-.113-1.411-.12-2.081.088h-.007a3.193 3.193 0 00-.671.302c-.968.563-2.164.767-2.967 1.577-.787.847-.739 2.012-.604 3.095h.033v.275c.013.095.028.19.04.282.41 2.19 1.5 4.2 1.84 6.412.065.843.203 1.932.309 2.618-.306-.091-.475-1.462-.544-1.007a38.196 38.196 0 00-3.565-5.25c-.853-1.004-1.697-2.06-2.712-2.894-.685-.528-.468-1.55-.537-2.302-.23-.926-.094-1.848.06-2.773.313-.963.418-1.968.846-2.893.653-.581.669-1.63 1.303-2.135.094.058.157.085.2.1l.068.008h.007c.09-.095-.888-1.116.02-.712.035-.537.854-.128.866-.597zm3.847 2.182c-.323.009-.574.13-.645.335-.114.33.273.755.866.96.594.205 1.168.109 1.282-.221.114-.33-.272-.762-.866-.967a1.842 1.842 0 00-.637-.107z"
        /> < title > { title } < / title > < / svg >
    }
}
