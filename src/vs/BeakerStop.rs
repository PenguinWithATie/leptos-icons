#[cfg(feature = "VsBeakerStop")]
use leptos::*;
#[cfg(feature = "VsBeakerStop")]
///This icon requires the feature `VsBeakerStop` to be enabled.
#[component]
pub fn BeakerStop(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2.99994 15.006H8.00746C7.62983 14.7234 7.29348 14.3888 7.00908 14.0126L2.99994 14.017L4.54094 11.006H5.99997L5.99997 11C5.99997 10.6597 6.03398 10.3273 6.09878 10.006H5.04894L6.89294 6.408L6.99994 6.193V2.036L8.99994 2.012V6.007V6.249L9.07058 6.38584C9.38043 6.25613 9.7061 6.15672 10.0439 6.09131L9.99994 6.006V2.006H10.9999V1.006H9.99394V1L9.53794 1.005H4.99994V2H5.99994V5.952L2.10594 13.561C2.03023 13.7133 1.99465 13.8825 2.00254 14.0524C2.01044 14.2224 2.06156 14.3875 2.15106 14.5321C2.24057 14.6768 2.3655 14.7962 2.51404 14.8792C2.66258 14.9621 2.82982 15.0057 2.99994 15.006ZM8.77769 7.67407C9.43548 7.23455 10.2089 7 11 7C12.0608 7 13.0782 7.42149 13.8283 8.17163C14.5785 8.92178 15 9.93913 15 11C15 11.7911 14.7654 12.5645 14.3259 13.2223C13.8864 13.8801 13.2616 14.3928 12.5307 14.6956C11.7998 14.9983 10.9955 15.0774 10.2196 14.9231C9.44366 14.7688 8.73102 14.3878 8.17161 13.8284C7.6122 13.269 7.23122 12.5563 7.07688 11.7804C6.92254 11.0045 7.00167 10.2001 7.30442 9.46924C7.60717 8.73833 8.11989 8.1136 8.77769 7.67407ZM8.87864 13.1213C9.44125 13.6839 10.2043 14 11 14C11.623 14.0018 12.2312 13.8095 12.74 13.45L8.55003 9.26001C8.19046 9.76883 7.99818 10.377 7.99998 11C7.99998 11.7956 8.31603 12.5587 8.87864 13.1213ZM9.25999 8.55005L13.4499 12.74C13.8095 12.2312 14.0018 11.623 14 11C14 10.2044 13.6839 9.44127 13.1213 8.87866C12.5587 8.31605 11.7956 8 11 8C10.3769 7.9982 9.7688 8.19048 9.25999 8.55005Z"
        /> < title > { title } < / title > < / svg >
    }
}
