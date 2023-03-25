#[cfg(feature = "SiImou")]
use leptos::*;
#[cfg(feature = "SiImou")]
///This icon requires the feature `SiImou` to be enabled.
#[component]
pub fn Imou(
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
        "M14.6426 9.1484c-1.5747 0-2.8555 1.279-2.8555 2.8516 0 1.5724 1.2808 2.8516 2.8555 2.8516 1.5748 0 2.8554-1.2792 2.8554-2.8516 0-1.5725-1.2806-2.8516-2.8554-2.8516zm-9.8184.0059c-1.3802 0-2.502 1.12-2.502 2.498v2.9532c0 .0886.0714.1601.1602.1601h.9727a.1598.1598 0 0 0 .1601-.1601v-2.9532c0-.6656.5425-1.207 1.209-1.207.6667 0 1.209.5414 1.209 1.207v2.9532c0 .0886.0733.1601.1621.1601h.9707a.1615.1615 0 0 0 .1621-.1601v-2.9532c0-.6656.5423-1.207 1.209-1.207.6665 0 1.209.5414 1.209 1.207v2.9532c0 .0886.0714.1601.1601.1601h.9727a.1598.1598 0 0 0 .1602-.1601v-2.9532c0-1.378-1.122-2.498-2.502-2.498-.7367 0-1.397.3209-1.8555.8281-.4582-.5072-1.1209-.8281-1.8574-.8281zm-4.664.1309A.1596.1596 0 0 0 0 9.4453v5.1602c0 .0888.0714.1601.1602.1601h.9628a.1614.1614 0 0 0 .1622-.1601V9.4453a.1615.1615 0 0 0-.1622-.1601H.1602zm18.289 0a.1612.1612 0 0 0-.162.1601v1.4492c0 .089.073.1602.162.1602h.963a.1596.1596 0 0 0 .16-.1602V9.4453a.1596.1596 0 0 0-.16-.1601h-.963zm4.4278 0a.1612.1612 0 0 0-.1622.1601v1.4492c0 .089.0732.1602.1622.1602h.9628A.1596.1596 0 0 0 24 10.8945V9.4453a.1596.1596 0 0 0-.1602-.1601h-.9628zm-8.2344 1.1464c.8663 0 1.5703.7035 1.5703 1.5684 0 .865-.704 1.5684-1.5703 1.5684-.8664 0-1.5723-.7035-1.5723-1.5684 0-.865.706-1.5684 1.5723-1.5684zm3.8066 1.2579a.1615.1615 0 0 0-.162.1601v.08c0 1.5443 1.2004 2.8615 2.746 2.92C22.6579 14.9108 24 13.6088 24 12v-.1504a.1598.1598 0 0 0-.1602-.1601h-.9628a.1615.1615 0 0 0-.1622.1601V12c0 .8651-.704 1.5684-1.5703 1.5684-.8663 0-1.5722-.7033-1.5722-1.5684v-.1504a.1597.1597 0 0 0-.1602-.1601h-.9629Z"
        /> < title > { title } < / title > < / svg >
    }
}
