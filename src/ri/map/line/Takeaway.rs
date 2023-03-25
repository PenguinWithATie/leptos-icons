#[cfg(feature = "RiMapLineTakeaway")]
use leptos::*;
#[cfg(feature = "RiMapLineTakeaway")]
///This icon requires the feature `RiMapLineTakeaway` to be enabled.
#[component]
pub fn Takeaway(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M16,1 C16.5522847,1 17,1.44771525 17,2 L17,2.999 L22,3 L22,9 L19.98,8.999 L22.7467496,16.595251 C22.9104689,17.0320314 23,17.5050658 23,17.9990113 C23,20.2081503 21.209139,21.9990113 19,21.9990113 C17.1367966,21.9990113 15.5711292,20.7251084 15.1264725,19.0007774 L10.8737865,19.0007613 C10.429479,20.7256022 8.86356525,22 7,22 C5.05513052,22 3.43445123,20.6119768 3.07453347,18.7725019 C2.43557576,18.4390399 2,17.770387 2,17 L2,4 C2,3.44771525 2.44771525,3 3,3 L10,3 C10.5522847,3 11,3.44771525 11,4 L11,12 C11,12.5128358 11.3860402,12.9355072 11.8833789,12.9932723 L12,13 L14,13 C14.5128358,13 14.9355072,12.6139598 14.9932723,12.1166211 L15,12 L15,3 L12,3 L12,1 L16,1 Z M7,16 C5.8954305,16 5,16.8954305 5,18 C5,19.1045695 5.8954305,20 7,20 C8.1045695,20 9,19.1045695 9,18 C9,16.8954305 8.1045695,16 7,16 Z M19,15.9990113 C17.8954305,15.9990113 17,16.8944418 17,17.9990113 C17,19.1035808 17.8954305,19.9990113 19,19.9990113 C20.1045695,19.9990113 21,19.1035808 21,17.9990113 C21,16.8944418 20.1045695,15.9990113 19,15.9990113 Z M17.852,8.999 L17,8.999 L17,12 C17,13.6568542 15.6568542,15 14,15 L12,15 C10.6941178,15 9.58311485,14.1656226 9.17102423,13.0009007 L3.99994303,13 L3.99994303,15.3542402 C4.73288889,14.523782 5.80527652,14 7,14 C8.86392711,14 10.4300871,15.2748927 10.8740452,17.0002597 L15.1256964,17.0002597 C15.5693048,15.2743991 17.135711,13.9990113 19,13.9990113 C19.2372818,13.9990113 19.469738,14.019672 19.6956678,14.0592925 L17.852,8.999 Z M9,8 L4,8 L4,11 L9,11 L9,8 Z M20,5 L17,5 L17,7 L20,7 L20,5 Z M9,5 L4,5 L4,6 L9,6 L9,5 Z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
