#[cfg(feature = "SiGatling")]
use leptos::*;
#[cfg(feature = "SiGatling")]
///This icon requires the feature `SiGatling` to be enabled.
#[component]
pub fn Gatling(
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
        "M19.1361 17.306c.6633-.393 1.2406-.9211 1.6827-1.5598h.7738a.7129.7129 0 0 0 .7124-.7124.7129.7129 0 0 0-.7124-.7124h-.0614c.1474-.4545.2457-.9212.2702-1.4125h.7615a.7129.7129 0 0 0 .7124-.7124.7129.7129 0 0 0-.7124-.7124h-.786c-.0492-.4913-.172-.9703-.344-1.4125h.1474a.7129.7129 0 0 0 .7124-.7123.7129.7129 0 0 0-.7124-.7124h-.958c-.9826-1.2283-2.5302-2.0512-4.262-2.0512h-1.216c-.7615 0-1.4984.1597-2.1617.4545H14.26c2.9478 0 5.3674 2.4196 5.3674 5.3674 0 2.7267-2.0511 4.9867-4.6919 5.3306-.1597.0123-.3193.0246-.4913.0246H8.9171a6.5941 6.5941 0 0 0 2.2477-2.0266c.0368.0122.086.0122.1228.0122h4.8148a.7129.7129 0 0 0 .7123-.7123.7129.7129 0 0 0-.7123-.7124H11.914a6.8244 6.8244 0 0 0 .3685-1.4125h5.5148a.7129.7129 0 0 0 .7124-.7124.7129.7129 0 0 0-.7124-.7124H12.344c-.0368-.4913-.1351-.958-.2702-1.4125h4.0532a.7129.7129 0 0 0 .7124-.7124.7129.7129 0 0 0-.7124-.7123h-4.6796c-1.1423-1.9284-3.2549-3.2303-5.65-3.2303h-.4667C3.1443 5.4289 1.1914 6.522 0 8.18a5.8463 5.8463 0 0 1 4.434-2.0266c2.5302 0 4.6919 1.609 5.5025 3.8567H6.9642C6.3746 9.261 5.4534 8.782 4.434 8.782c-1.781.0246-3.218 1.4616-3.218 3.2426s1.4493 3.2303 3.2303 3.2303c1.0563 0 1.9897-.5036 2.5793-1.2897H3.955c0-1.4248 1.1545-2.567 2.567-2.567h3.7462c.0245.2088.0368.4176.0368.6264 0 3.2303-2.6284 5.8587-5.8587 5.8587-1.7073 0-3.2426-.737-4.3235-1.9038 1.2037 1.5722 3.0952 2.5916 5.22 2.5916h9.6296c4.9867 0 9.0276-.2947 9.0276-.6755 0-.2457-1.9775-.479-4.8639-.5896Z"
        /> < title > { title } < / title > < / svg >
    }
}
