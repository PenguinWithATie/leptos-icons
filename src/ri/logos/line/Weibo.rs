#[cfg(feature = "RiLogosLineWeibo")]
use leptos::*;
#[cfg(feature = "RiLogosLineWeibo")]
///This icon requires the feature `RiLogosLineWeibo` to be enabled.
#[component]
pub fn Weibo(
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
        "M20.194 14.197c0 3.362-4.53 6.424-9.926 6.424C5.318 20.62 1 18.189 1 14.534c0-1.947 1.18-4.087 3.24-6.088 2.832-2.746 6.229-4.033 7.858-2.448.498.482.723 1.122.719 1.858 1.975-.576 3.65-.404 4.483.752.449.623.532 1.38.326 2.207 1.511.61 2.568 1.77 2.568 3.382zm-4.44-2.07c-.386-.41-.4-.92-.198-1.41.208-.508.213-.812.12-.94-.264-.368-1.533-.363-3.194.311a2.043 2.043 0 0 1-.509.14c-.344.046-.671.001-.983-.265-.419-.359-.474-.855-.322-1.316.215-.67.18-1.076.037-1.215-.186-.18-.777-.191-1.659.143-1.069.405-2.298 1.224-3.414 2.306C3.925 11.54 3 13.218 3 14.534c0 2.242 3.276 4.087 7.268 4.087 4.42 0 7.926-2.37 7.926-4.424 0-.738-.637-1.339-1.673-1.652-.394-.113-.536-.171-.767-.417zm7.054-1.617a1 1 0 0 1-1.936-.502 4 4 0 0 0-4.693-4.924 1 1 0 1 1-.407-1.958 6 6 0 0 1 7.036 7.384z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
