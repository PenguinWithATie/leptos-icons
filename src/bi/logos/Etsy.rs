#[cfg(feature = "BiLogosEtsy")]
use leptos::*;
#[cfg(feature = "BiLogosEtsy")]
///This icon requires the feature `BiLogosEtsy` to be enabled.
#[component]
pub fn Etsy(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11.139 14.058c-.438 0-.693-.294-.693-1.058v-2.696l1.456.112.074-.694-1.496.074v-1.53h-.438c-.216 1.201-.759 1.638-1.488 1.712v.364h.869v3.062c0 .836.588 1.271 1.313 1.271.585 0 1.202-.255 1.418-.763l-.217-.255c-.108.18-.402.401-.798.401zm-3.494-.803c-.294.69-.767.729-1.24.729h-1.53c-.508 0-.729-.186-.729-.62v-2.258s1.092 0 1.457.039c.291.035.438.109.546.473l.108.511h.438l-.039-1.309.074-1.278h-.434l-.147.585c-.073.399-.147.438-.546.508-.508.038-1.457.038-1.457.038V7.979c0-.112 0-.147.185-.147h2.291c.402 0 .62.326.763.98l.147.512h.399c.039-1.457.074-2.074.074-2.074s-.981.105-1.562.105H3.53L2 7.324v.399l.508.107c.367.074.475.147.475.473 0 0 .039.984.039 2.619 0 1.639-.039 2.623-.039 2.623 0 .291-.108.399-.475.473L2 14.131v.396l1.565-.035h2.623c.581 0 1.965.035 1.965.035.034-.361.216-1.964.255-2.146H8.04l-.395.874zm6.04-2.622c0-.474.437-.655.875-.655.36 0 .653.147.729.325l.255.729.361-.035c0-.364.038-.838.111-1.201-.328-.147-.983-.221-1.421-.221-.984 0-1.747.441-1.747 1.387 0 1.674 2.44 1.31 2.44 2.549 0 .399-.256.729-.876.729-.581 0-.837-.294-.945-.585l-.29-.693h-.369c.04.476.074.948 0 1.382 0 0 .767.294 1.531.294 1.022 0 1.858-.507 1.858-1.457 0-1.674-2.512-1.421-2.512-2.548zm6.369-.945v.364l.259.074c.252.07.36.178.36.363 0 .108-.034.144-.073.291-.108.291-.802 2.003-1.162 2.767a75.566 75.566 0 0 1-1.093-2.949c-.035-.073-.035-.108-.035-.146 0-.148.105-.292.361-.326l.329-.074v-.364l-1.346.073-1.096-.039v.368l.187.069c.251.074.287.11.469.512.693 1.53 1.457 3.529 1.716 4.15-.584 1.019-1.204 1.309-1.677 1.309-.291 0-.399-.147-.435-.329l-.146-.763-.4.035c-.073.477-.147.984-.255 1.383.287.182.616.294 1.053.293.691 0 1.675-.182 2.623-2.332l1.601-3.747c.145-.293.182-.329.546-.476l.183-.144v-.362l-.945.034-1.024-.034z"
        /> < title > { title } < / title > < / svg >
    }
}
