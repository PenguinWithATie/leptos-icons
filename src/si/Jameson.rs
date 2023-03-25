#[cfg(feature = "SiJameson")]
use leptos::*;
#[cfg(feature = "SiJameson")]
///This icon requires the feature `SiJameson` to be enabled.
#[component]
pub fn Jameson(
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
        "M18.81 17.564c-.405 1.031-1.007 2.505-1.683 3.574 0 .012 0 .024.013.012.737-.59 1.486-1.732 1.891-2.763.258-.651.823-2.002 1.044-2.223.013-.013 0-.025-.012-.037-.43-.123-1.339-.234-1.867-2.236 0 0-2.678-9.985-3.046-11.385-.295-1.094-.037-1.757.356-2.199.012-.012 0-.025-.012-.012-.712.22-1.191 1.29-.86 2.505.369 1.388 3.01 11.226 3.01 11.226.171.651.405 1.167.663 1.536.258.368.614.638.97.712.037.012.086.012.123.012.012 0 .024.025.012.037-.025.013-.037.037-.061.062-.172.208-.357.65-.553 1.154zM14.352.024l.012.05v.012c-.59.405-1.167 1.486-.835 2.727.368 1.388 1.94 7.246 2.493 9.285l.602 2.223c.368 1.4.897 2.174 1.621 2.272 0 0 .012 0 .012.013l.025.049v.012c-.368.369-.676 1.265-1.069 2.1-.393.848-1.215 3.415-4.36 4.619-3.427 1.302-6.902.479-8.364-1.941-.958-1.572-.725-3.734.835-4.606 1.351-.761 2.813-.258 3.366.86.528 1.08.16 2.235-.27 2.616-.087.074 0 .233.208.368.909.627 1.953.59 2.948.062 1.277-.676 1.744-1.842 1.068-4.323-.54-2.04-2.972-11.08-3.34-12.48-.332-1.24-1.376-1.89-2.088-1.94 0 0-.013 0-.013-.012l-.012-.05s0-.012.012-.012L14.39 0s.012 0 .012.012zM9.795 20.083c.16.086.356.11.54.073.013 0 .013-.012.013-.012a3.249 3.249 0 0 0-.172-2.493c-.369-.75-.91-1.192-1.523-1.4-.013 0-.013.012-.013.012.393.27.725.639.958 1.105.443.91.43 1.88.16 2.629a.06.06 0 0 0 .025.073Z"
        /> < title > { title } < / title > < / svg >
    }
}
