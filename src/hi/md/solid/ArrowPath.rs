#[cfg(feature = "HiMdSolidArrowPath")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowPath")]
///This icon requires the feature `HiMdSolidArrowPath` to be enabled.
#[component]
pub fn ArrowPath(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.3124 11.4236C14.5262 14.3576 11.5104 16.0988 8.5763 15.3127C7.60728 15.053 6.77024 14.5514 6.11052 13.8904L5.79898 13.5789L8.23221 13.5785C8.64642 13.5784 8.98215 13.2425 8.98208 12.8283C8.982 12.4141 8.64616 12.0784 8.23194 12.0785L3.98851 12.0792C3.78959 12.0792 3.59884 12.1583 3.45821 12.299C3.31758 12.4397 3.2386 12.6305 3.23864 12.8294L3.23954 17.0712C3.23962 17.4854 3.57548 17.8211 3.98969 17.821C4.40391 17.821 4.73962 17.4851 4.73954 17.0709L4.73903 14.6403L5.04888 14.9501C5.88991 15.7926 6.95759 16.4318 8.18808 16.7615C11.9223 17.7621 15.7607 15.5461 16.7613 11.8118C16.8685 11.4117 16.6311 11.0005 16.231 10.8932C15.8309 10.786 15.4196 11.0235 15.3124 11.4236ZM16.5413 7.70119C16.682 7.56051 16.761 7.36971 16.7609 7.17078L16.7603 2.92883C16.7602 2.51462 16.4243 2.17889 16.0101 2.17896C15.5959 2.17902 15.2602 2.51487 15.2603 2.92908L15.2606 5.35995L14.9508 5.05013C14.1098 4.20771 13.0419 3.56827 11.8115 3.23859C8.07728 2.238 4.23892 4.45407 3.23833 8.18834C3.13112 8.58844 3.36856 8.99969 3.76866 9.1069C4.16876 9.2141 4.58001 8.97666 4.68722 8.57657C5.4734 5.6425 8.48925 3.9013 11.4233 4.68748C12.3924 4.94714 13.2294 5.44881 13.8892 6.10979L14.2003 6.4209L11.7683 6.4209C11.3541 6.4209 11.0183 6.75669 11.0183 7.1709C11.0183 7.58512 11.3541 7.9209 11.7683 7.9209H16.0109C16.2099 7.9209 16.4007 7.84187 16.5413 7.70119Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
