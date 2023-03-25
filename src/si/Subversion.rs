#[cfg(feature = "SiSubversion")]
use leptos::*;
#[cfg(feature = "SiSubversion")]
///This icon requires the feature `SiSubversion` to be enabled.
#[component]
pub fn Subversion(
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
        "M24 20.753v-6.306c-3.285 1.296-7.362 2.556-12.23 3.786-4.534 1.145-8.458 1.97-11.77 2.475v.045h24zM0 14.078v5.133c3.738-.55 7.116-1.206 10.13-1.967 2.962-.748 5.245-1.475 6.847-2.18 1.602-.703 2.34-1.297 2.22-1.78-.107-.42-.846-.635-2.217-.645-.703.01-1.67.06-2.904.15-1.236.09-2.774.234-4.61.426-2.85.304-5.216.537-7.1.694-.896.075-1.685.132-2.366.17zm1.035 2.95c.06 0 .114.025.16.07.046.046.07.103.07.166 0 .066-.024.12-.07.168-.047.045-.104.066-.164.066-.032 0-.064-.006-.092-.018-.03-.012-.054-.03-.075-.05-.023-.014-.04-.044-.05-.074 0-.015-.016-.045-.016-.09 0-.06.03-.12.075-.165s.105-.06.18-.06zm.81 0c.063 0 .117.025.165.07.045.046.066.103.066.166 0 .066-.022.12-.067.168-.06.045-.106.066-.18.066-.03 0-.06-.006-.09-.018s-.06-.03-.076-.05c-.03-.014-.045-.044-.06-.074-.015-.015-.015-.045-.015-.09 0-.06.014-.12.06-.165s.104-.06.164-.06zm-.81-1.51c.06 0 .114.022.16.07.046.045.07.1.07.165 0 .064-.024.12-.07.165s-.1.07-.164.07c-.065 0-.122-.024-.167-.07-.045-.045-.07-.102-.07-.165 0-.067.016-.123.06-.168s.106-.068.166-.068zm.81 0c.063 0 .117.022.165.07.045.045.066.1.066.165 0 .064-.022.12-.067.165-.06.045-.106.07-.18.07s-.12-.024-.166-.07c-.045-.045-.075-.102-.075-.165 0-.067.014-.123.06-.168s.104-.068.164-.068zM24 4.597V9.41c-1.635.1-3.68.277-6.138.534-2.49.27-4.52.48-6.093.615-1.576.15-2.713.226-3.41.24-1.363.03-2.09-.15-2.195-.554-.105-.45.705-1.05 2.445-1.77 1.74-.735 4.05-1.47 6.9-2.19 2.505-.63 5.34-1.185 8.49-1.65zm-.855-1.35c-3.255.605-6.627 1.35-10.114 2.23C7.587 6.852 3.244 8.22 0 9.573V3.248h23.146z"
        /> < title > { title } < / title > < / svg >
    }
}
