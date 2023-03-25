#[cfg(feature = "SiVirginmedia")]
use leptos::*;
#[cfg(feature = "SiVirginmedia")]
///This icon requires the feature `SiVirginmedia` to be enabled.
#[component]
pub fn Virginmedia(
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
        "M5.225 5.021c-1.098 0-2.067.346-2.883 1.024C1.416 6.813 0 8.635 0 12c0 3.366 1.416 5.187 2.342 5.955.816.678 1.785 1.024 2.883 1.024 1.629 0 3.229-.75 4.271-1.401 2.828-1.769 5.082-5.143 5.082-5.143 1.093 1.502 2.081 2.494 3.199 3.21.687.419 1.723.914 2.787.914.725 0 1.371-.22 1.904-.677C23.487 15.008 24 13.658 24 12s-.513-3.008-1.532-3.882c-.532-.457-1.18-.677-1.904-.677-1.064 0-2.1.495-2.787.914-1.118.716-2.106 1.708-3.2 3.21 0 0-2.253-3.374-5.08-5.143-1.043-.651-2.643-1.401-4.272-1.401Zm0 .513c1.51 0 3.011.722 4 1.324 1.69 1.084 3.25 2.647 5.036 5.142-1.785 2.494-3.346 4.057-5.037 5.142-.988.602-2.49 1.323-3.999 1.323-.974 0-1.833-.305-2.554-.904C1.085 16.243.514 13.916.514 12s.571-4.243 2.157-5.561c.721-.6 1.58-.905 2.554-.905zm15.34 2.42c.6 0 1.13.188 1.575.558.993.828 1.347 2.273 1.347 3.488 0 1.215-.354 2.66-1.347 3.488-.444.37-.974.557-1.576.557-.945 0-1.889-.454-2.515-.835-1.067-.685-2.01-1.63-3.154-3.21 1.144-1.581 2.087-2.526 3.154-3.21.626-.381 1.57-.836 2.515-.836zm-17.6 3.505-.01.013.01-.01v-.003z"
        /> < title > { title } < / title > < / svg >
    }
}
