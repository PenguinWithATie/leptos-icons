#[cfg(feature = "SiAzuredataexplorer")]
use leptos::*;
#[cfg(feature = "SiAzuredataexplorer")]
///This icon requires the feature `SiAzuredataexplorer` to be enabled.
#[component]
pub fn Azuredataexplorer(
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
        "M23.1765.0176a.8032.8032 0 00-.0235.002h-8.0679L7.5426 7.562l8.8666 8.8667 7.5895-7.5894V.8222a.8032.8032 0 00-.8222-.8046zm-21.245.002a.8032.8032 0 00-.5644 1.365l5.4685 5.4705L13.671.0195zm14.8937 2.326c.137 0 .2721.0533.377.1582l.9374.9355a.535.535 0 010 .7577l-.9374.9355a.5326.5326 0 01-.7558 0l-.9375-.9355a.535.535 0 010-.7577l.9375-.9355a.5353.5353 0 01.3788-.1582zM13.4426 5.734c.1369 0 .274.0513.3789.1562l.9355.9374a.5326.5326 0 010 .7559l-.9355.9374a.535.535 0 01-.7578 0l-.9355-.9374a.5326.5326 0 010-.7559l.9355-.9374a.5332.5332 0 01.379-.1562zm6.773.002a.5355.5355 0 01.3731.1581l.9296.9296a.5355.5355 0 010 .7617l-.9296.9297a.5355.5355 0 01-.7617 0l-.9296-.9297a.5355.5355 0 010-.7617l.9296-.9296a.5355.5355 0 01.3887-.1582zm-3.3845 3.3786c.1369 0 .272.0533.377.1582l.9374.9355a.535.535 0 010 .7578l-.9375.9374a.5326.5326 0 01-.7558 0l-.9374-.9374a.535.535 0 010-.7578l.9374-.9355a.5353.5353 0 01.3789-.1582zm-11.015.7226a.3799.3799 0 00-.2695.1114L.1172 15.378a.3808.3808 0 000 .539l1.1562 1.1562c.149.149.388.149.537 0l5.4314-5.4314a.3785.3785 0 000-.537L6.0856 9.9487a.3799.3799 0 00-.2695-.1114zm18.1826.416l-6.8825 6.8825 5.4724 5.4704a.8032.8032 0 001.41-.5625zM9.1792 13.265a.382.382 0 00-.2695.1133L.1113 22.1746a.3808.3808 0 000 .539l1.1562 1.1562c.149.149.39.149.539 0l8.7964-8.7984a.3808.3808 0 000-.539l-1.1542-1.1542a.382.382 0 00-.2695-.1133zm3.365 3.3475a.3799.3799 0 00-.2695.1113L6.8434 22.155a.3785.3785 0 000 .5371l1.1562 1.1562c.149.149.39.149.539 0l5.4294-5.4313a.3785.3785 0 000-.5371l-1.1562-1.1562a.3755.3755 0 00-.2676-.1113Z"
        /> < title > { title } < / title > < / svg >
    }
}
