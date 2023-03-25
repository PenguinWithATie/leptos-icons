#[cfg(feature = "SiPrivateinternetaccess")]
use leptos::*;
#[cfg(feature = "SiPrivateinternetaccess")]
///This icon requires the feature `SiPrivateinternetaccess` to be enabled.
#[component]
pub fn Privateinternetaccess(
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
        "M11.998 0C7.6513 0 4.129 3.5034 4.129 7.8242v.8692h-.9747c-1.022 0-1.8496.8238-1.8496 1.8398v7.1816c.002.4784.0315.8267.2187 1.0801a.7482.7482 0 0 0 .3717.26c.1378.0425.2532.0629.3842.074.0783.0063.163.01.252.0098h1.5761c.0462.6196.077 1.285.1074 1.8828.0418.647.081 1.2944.1192 1.9414.0007.1005.0115.3301.1387.5508.125.2227.3877.4214.8125.418h3.9785l2.0664-2.1915h1.543L14.871 24h3.5684c.3073-.0014.5901.0066.834-.1484.243-.1585.3884-.46.459-.9844.0519-.3859.0826-1.1097.1073-1.8457.0008-.6137.0336-1.2722.0567-1.8985h1.5703c.1886-.0007.3493.008.53-.0197.1808-.0278.347-.098.472-.2322.197-.2234.2237-.542.2265-1.0195v-7.3184c-.0014-1.016-.8302-1.8391-1.8515-1.8398-.3235.0012-.65 0-.9747 0v-.8692C19.8691 3.5034 16.3455 0 11.9981 0zm-.0117 2.8652c2.6715 0 4.836 2.1607 4.836 4.8262v1.414H7.1484v-1.414c0-2.6655 2.1665-4.8262 4.838-4.8262zm-1.5879 1.7305a.8935.8935 0 0 0-.8945.8945.8935.8935 0 0 0 .8945.8926.8935.8935 0 0 0 .8926-.8926.8935.8935 0 0 0-.8926-.8945zm3.1758 0a.8935.8935 0 0 0-.8926.8945.8935.8935 0 0 0 .8926.8926.8935.8935 0 0 0 .8946-.8926.8935.8935 0 0 0-.8946-.8945zM9.957 7.6309v.5136h4.3125V7.631zm-5.5879 1.957h.25v9.1074h14.7852v.4902H4.5957v-.3203h-.2266zm15.0625 0h.252v9.2773h-.252zm-7.5312 3.4043c.7465 0 1.3627.537 1.5176 1.248h-.3848c-.1465-.5093-.5984-.8782-1.1328-.8789-.5344.0007-.985.3696-1.1309.879h-.3867c.1549-.7112.7711-1.2474 1.5176-1.2481zm-1.6309 1.5039h3.2344v2.4121h-3.2344zM21.957 19.375c-.0764.0035-.1565.0078-.2441.0078h-1.5703c-.0208.3892-.0358 1.004-.0567 1.6387h.3633c.8338.0007 1.5078-.6765 1.5078-1.5117zm-20.0273.002v.1347c0 .8338.6682 1.5098 1.4941 1.5098h.4903l-.1016-1.629H2.2539c-.1182 0-.2256-.0038-.3242-.0155Z"
        /> < title > { title } < / title > < / svg >
    }
}
