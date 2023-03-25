#[cfg(feature = "VsVariableGroup")]
use leptos::*;
#[cfg(feature = "VsVariableGroup")]
///This icon requires the feature `VsVariableGroup` to be enabled.
#[component]
pub fn VariableGroup(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.387 11.523a.402.402 0 0 1 .593-.367c.058.031.11.065.157.102.047.036.088.07.125.101a.177.177 0 0 0 .117.047c.052 0 .12-.04.203-.117.083-.078.175-.182.273-.313.1-.13.201-.268.305-.414.104-.146.2-.294.29-.445l.226-.39c.062-.11.107-.199.133-.266a15.33 15.33 0 0 0-.133-.524 15.384 15.384 0 0 1-.133-.523 3.72 3.72 0 0 0-.133-.422 1.04 1.04 0 0 0-.187-.313.656.656 0 0 0-.266-.187 1.374 1.374 0 0 0-.375-.07 1.628 1.628 0 0 0-.328.031v-.195L7.69 7a2.345 2.345 0 0 1 .461.734c.052.13.097.263.133.399.037.135.076.283.117.445.078-.115.175-.26.29-.438a4.49 4.49 0 0 1 .398-.523c.15-.172.31-.315.476-.43A1.02 1.02 0 0 1 10.089 7c.13 0 .247.034.351.101.105.068.157.175.157.32 0 .282-.141.423-.422.423a.608.608 0 0 1-.29-.07.608.608 0 0 0-.288-.071c-.1 0-.203.05-.313.148a2.3 2.3 0 0 0-.312.352 9.5 9.5 0 0 0-.485.734l.446 1.852a1.56 1.56 0 0 0 .093.344.669.669 0 0 0 .094.171.184.184 0 0 0 .125.079.37.37 0 0 0 .211-.086 2.14 2.14 0 0 0 .43-.47c.052-.077.093-.15.125-.218l.187.094a2.025 2.025 0 0 1-.219.367 3.775 3.775 0 0 1-.351.422 3.38 3.38 0 0 1-.406.36c-.141.104-.269.153-.383.148a.397.397 0 0 1-.281-.102 1.491 1.491 0 0 1-.204-.234 1.599 1.599 0 0 1-.132-.36 8.263 8.263 0 0 1-.118-.507 34.16 34.16 0 0 1-.101-.532 2.212 2.212 0 0 0-.11-.414l-.203.375a4.489 4.489 0 0 1-.28.453c-.11.157-.222.316-.337.477a2.46 2.46 0 0 1-.375.422c-.135.12-.265.221-.39.305A.66.66 0 0 1 5.91 12a.539.539 0 0 1-.36-.133.454.454 0 0 1-.163-.344zm6.11.477c.28-.36.496-.748.648-1.164a3.87 3.87 0 0 0 .226-1.32c0-.47-.075-.912-.226-1.329A4.57 4.57 0 0 0 11.495 7h.734a3.77 3.77 0 0 1 .922 2.515c0 .474-.073.917-.218 1.329-.146.411-.38.796-.704 1.156h-.734zM3.77 12a3.373 3.373 0 0 1-.704-1.149 3.97 3.97 0 0 1-.218-1.336c0-.953.307-1.791.922-2.515h.726a4.132 4.132 0 0 0-.64 1.18 4.205 4.205 0 0 0-.227 1.335A3.929 3.929 0 0 0 4.496 12H3.77z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 1H.5l-.5.5v13l.5.5h15l.5-.5v-13l-.5-.5zM15 14H1V5h14v9zm0-10H1V2h14v2z" />
        < title > { title } < / title > < / svg >
    }
}
