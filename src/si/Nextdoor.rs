#[cfg(feature = "SiNextdoor")]
use leptos::*;
#[cfg(feature = "SiNextdoor")]
///This icon requires the feature `SiNextdoor` to be enabled.
#[component]
pub fn Nextdoor(
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
        "M14.65 9.997a.069.069 0 0 0-.07.069v1.415c-.123-.177-.42-.37-.805-.37-.745 0-1.316.659-1.316 1.445 0 .787.571 1.447 1.316 1.447.386 0 .682-.194.806-.372v.221c0 .05.04.09.09.09h.607a.07.07 0 0 0 .07-.07v-3.806a.069.069 0 0 0-.07-.069zm-3.913.404a.07.07 0 0 0-.069.07c0 .779.064.7-.504.7a.09.09 0 0 0-.09.09v.486c0 .05.04.089.09.089h.504v1.136c0 .676.476 1.003 1.07 1.003.183 0 .32-.017.434-.046a.07.07 0 0 0 .052-.067v-.526a.07.07 0 0 0-.086-.066.984.984 0 0 1-.227.023c-.33 0-.476-.133-.476-.47v-.987h.608a.07.07 0 0 0 .07-.069v-.527a.069.069 0 0 0-.07-.069h-.608v-.701a.069.069 0 0 0-.069-.07zm-8.396.676c-.516 0-.955.236-1.201.598-.02.03-.055.095-.102.095-.226.002-.24-.276-.247-.524a.07.07 0 0 0-.069-.066l-.653-.004a.07.07 0 0 0-.069.07c.014.606.126 1.018.86 1.181.04.01.068.045.068.087v1.36c0 .037.03.068.069.068h.634a.07.07 0 0 0 .069-.07V12.47c0-.312.221-.667.64-.667.4 0 .642.355.642.667v1.404c0 .038.03.069.069.069h.634a.07.07 0 0 0 .069-.07v-1.508c0-.72-.616-1.287-1.413-1.287zm3.207.033c-.851 0-1.472.626-1.472 1.446 0 .876.65 1.431 1.483 1.447.655.012 1.09-.363 1.194-.494a.068.068 0 0 0-.015-.097l-.435-.34c-.047-.047-.084-.021-.112.001-.07.057-.203.22-.626.237-.37.015-.7-.205-.745-.576h2.03a.07.07 0 0 0 .069-.065c.006-.082.006-.142.006-.196 0-.897-.644-1.363-1.377-1.363zm11.652 0c-.812 0-1.472.637-1.472 1.446 0 .81.66 1.447 1.472 1.447.812 0 1.472-.638 1.472-1.447s-.66-1.446-1.472-1.446zm3.229 0c-.812 0-1.472.637-1.472 1.446 0 .81.66 1.447 1.472 1.447.812 0 1.472-.638 1.472-1.447s-.66-1.446-1.472-1.446zm3.314.028a.745.745 0 0 0-.695.476v-.374a.069.069 0 0 0-.069-.069h-.628a.069.069 0 0 0-.07.07v2.632a.07.07 0 0 0 .07.069h.628a.07.07 0 0 0 .07-.07v-1.255c0-.454.24-.737.604-.737.092 0 .175.013.26.035A.069.069 0 0 0 24 11.85v-.624a.07.07 0 0 0-.056-.068.938.938 0 0 0-.201-.02zm-16.666.033a.069.069 0 0 0-.058.108l.88 1.305L7 13.832a.07.07 0 0 0 .056.11h.745a.068.068 0 0 0 .056-.03l.564-.79.563.79a.069.069 0 0 0 .056.03h.74a.069.069 0 0 0 .057-.11l-.899-1.248.88-1.305a.069.069 0 0 0-.058-.108h-.738a.07.07 0 0 0-.058.03l-.548.818-.549-.817a.07.07 0 0 0-.057-.03zm-1.552.565c.286 0 .566.155.633.482h-1.31c.073-.338.392-.482.677-.482zm8.412.067c.42 0 .705.321.705.753 0 .433-.285.754-.705.754s-.705-.321-.705-.754c0-.432.285-.753.705-.753zm3.263.016c.403 0 .694.31.694.737s-.291.737-.694.737c-.403 0-.7-.31-.7-.737 0-.426.297-.737.7-.737zm3.229 0c.403 0 .694.31.694.737s-.291.737-.694.737c-.403 0-.7-.31-.7-.737 0-.426.297-.737.7-.737z"
        /> < title > { title } < / title > < / svg >
    }
}
