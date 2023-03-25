#[cfg(feature = "TbChartDonut4")]
use leptos::*;
#[cfg(feature = "TbChartDonut4")]
///This icon requires the feature `TbChartDonut4` to be enabled.
#[component]
pub fn ChartDonut4(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-chart-donut-4" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8.848 14.667l-3.348 2.833"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 3v5m4 4h5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14.219 15.328l2.781 4.172"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> < title > { title } < / title > < /
        svg >
    }
}
