#[cfg(feature = "SiLibrarything")]
use leptos::*;
#[cfg(feature = "SiLibrarything")]
///This icon requires the feature `SiLibrarything` to be enabled.
#[component]
pub fn Librarything(
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
        "M8.16 0h7.68c2.66 0 3.508 0 4.708.4 1.4.5 2.552 1.65 3.052 3.05.4 1.2.4 2.05.4 4.71v7.68c0 2.66 0 3.51-.4 4.71a5.04 5.04 0 0 1-3.05 3.05c-1.2.4-2.05.4-4.71.4H8.16c-2.66 0-3.51 0-4.71-.4A5.043 5.043 0 0 1 .4 20.55C0 19.35 0 18.5 0 15.84V8.16C0 5.5 0 4.65.4 3.45A5.04 5.04 0 0 1 3.45.4C4.65 0 5.5 0 8.16 0zm2.834 6.682v-.297l-.023-.321c0-.092-.03-.23-.091-.412a1.552 1.552 0 0 1-.092-.481c0-.092.038-.168.115-.23a5.55 5.55 0 0 0 .24-.194c.1-.084.24-.126.424-.126.107 0 .282.015.527.046.214 0 .374-.046.481-.138.046-.03.13-.091.252-.183s.183-.199.183-.32c0-.2-.11-.36-.332-.482-.221-.122-.5-.183-.836-.183l-.94.069c-.213 0-.618.05-1.214.149-.595.099-1.061.149-1.397.149a5.04 5.04 0 0 1-.7-.08 5.174 5.174 0 0 0-.79-.08c-.168.06-.34.118-.515.171-.176.053-.263.134-.263.24l.045.207c.092.198.23.35.413.458l.183.091c.092.03.168.046.23.046.075 0 .19-.007.343-.023.122 0 .237.03.343.092l.252.229c.123.137.21.271.264.4.053.13.08.249.08.356 0 .076-.015.206-.046.39a2.86 2.86 0 0 0-.046.434c0 .138.035.336.104.596.068.26.103.473.103.641-.016.26-.023.52-.023.78v.778c0 .198.015.458.046.779.03.32.045.572.045.755l-.045.962-.023 1.535-.07 1.26c0 .168.031.39.093.664.06.275.091.496.091.664 0 .168-.023.382-.069.642-.045.26-.068.465-.068.618 0 .076.007.176.023.298a.808.808 0 0 1 0 .275.56.56 0 0 1-.126.263 37.9 37.9 0 0 1-.218.264c-.137.168-.29.297-.458.389-.168.092-.405.145-.71.16-.306.016-.527.08-.665.195-.137.115-.206.202-.206.263 0 .397.367.596 1.1.596l.16-.023a35.207 35.207 0 0 1 1.317-.137c.222-.016.478-.023.768-.023h.275c.275.015.55.023.825.023h2.2c.213-.016.415-.035.606-.058a4.37 4.37 0 0 1 .516-.034c.198 0 .473.05.825.149.35.1.649.133.893.103.015 0 .103-.023.264-.069a1.69 1.69 0 0 1 .47-.068c.014 0 .118.011.308.034.191.023.348.034.47.034.061 0 .168-.015.32-.046.383-.076.65-.221.803-.435a.767.767 0 0 0 .16-.435 3.704 3.704 0 0 1-.069-.435 2.77 2.77 0 0 1 0-.504c.077-.153.16-.305.252-.458.077-.199.13-.47.16-.813.031-.344.058-.573.081-.687a2.79 2.79 0 0 1 .195-.527c.107-.237.16-.44.16-.607a.974.974 0 0 0-.114-.481c-.077-.138-.176-.206-.298-.206-.077 0-.199.068-.367.206-.076.06-.141.175-.195.343a2.778 2.778 0 0 1-.126.344c-.183.275-.366.542-.55.802a3.768 3.768 0 0 0-.229.435c-.152.32-.274.542-.366.664-.03.046-.153.16-.367.344a1.124 1.124 0 0 0-.217.24c-.07.1-.165.256-.287.47l-.39.206-.274.069a1.894 1.894 0 0 0-.367.114c-.152.061-.267.1-.343.115a1.571 1.571 0 0 1-.298.022c-.138 0-.344-.022-.619-.068a4.242 4.242 0 0 0-.664-.069l-.481.023c-.55 0-.955-.298-1.215-.893-.168-.382-.297-.978-.39-1.787a2.327 2.327 0 0 1-.022-.366c0-.123.008-.329.023-.619 0-.076.019-.237.057-.48a4.07 4.07 0 0 0 .057-.596v-1.604c0-.137.008-.313.023-.527.016-.213.023-.366.023-.458a2.14 2.14 0 0 0-.023-.343c0-.092-.03-.222-.091-.39a1.244 1.244 0 0 1-.092-.389c0-.076.015-.183.046-.32.03-.138.046-.245.046-.321 0-.168-.008-.397-.023-.688a13.83 13.83 0 0 1-.023-.687c0-.198.03-.442.092-.733a3.6 3.6 0 0 0 .091-.71z"
        /> < title > { title } < / title > < / svg >
    }
}
