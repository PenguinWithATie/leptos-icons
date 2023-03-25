#[cfg(feature = "SiMacos")]
use leptos::*;
#[cfg(feature = "SiMacos")]
///This icon requires the feature `SiMacos` to be enabled.
#[component]
pub fn Macos(
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
        "M0 14.727h.941v-2.453c0-.484.318-.835.771-.835.439 0 .71.276.71.722v2.566h.915V12.25c0-.48.31-.812.764-.812.46 0 .718.28.718.77v2.518h.94v-2.748c0-.801-.517-1.334-1.307-1.334-.578 0-1.054.31-1.247.805h-.023c-.147-.514-.552-.805-1.118-.805-.545 0-.968.306-1.142.771H.903v-.695H0v4.006zm7.82-.646c-.408 0-.68-.208-.68-.537 0-.318.26-.522.714-.552l.926-.057v.307c0 .483-.427.839-.96.839zm-.284.71c.514 0 1.017-.268 1.248-.703h.018v.639h.908v-2.76c0-.804-.647-1.33-1.64-1.33-1.021 0-1.66.537-1.701 1.285h.873c.06-.332.344-.548.79-.548.464 0 .748.242.748.662v.287l-1.058.06c-.976.061-1.524.488-1.524 1.199 0 .721.564 1.209 1.338 1.209zm6.305-2.642c-.065-.843-.719-1.512-1.777-1.512-1.164 0-1.92.805-1.92 2.087 0 1.3.756 2.082 1.928 2.082 1.005 0 1.697-.59 1.772-1.485h-.888c-.087.453-.397.725-.873.725-.597 0-.982-.483-.982-1.322 0-.824.381-1.323.975-1.323.502 0 .8.321.876.748h.889zm2.906-2.967c-1.591 0-2.589 1.085-2.589 2.82 0 1.735.998 2.816 2.59 2.816 1.586 0 2.584-1.081 2.584-2.816 0-1.735-.997-2.82-2.585-2.82zm0 .832c.971 0 1.591.77 1.591 1.988 0 1.213-.62 1.984-1.59 1.984-.976 0-1.592-.77-1.592-1.984 0-1.217.616-1.988 1.591-1.988zm2.982 3.178c.042 1.006.866 1.626 2.12 1.626 1.32 0 2.151-.65 2.151-1.686 0-.813-.469-1.27-1.576-1.523l-.627-.144c-.67-.158-.945-.37-.945-.733 0-.453.415-.756 1.032-.756.623 0 1.05.306 1.096.817h.93c-.023-.96-.817-1.61-2.019-1.61-1.187 0-2.03.653-2.03 1.62 0 .78.477 1.263 1.482 1.494l.707.166c.688.163.967.39.967.782 0 .454-.457.779-1.115.779-.665 0-1.167-.329-1.228-.832h-.945z"
        /> < title > { title } < / title > < / svg >
    }
}
