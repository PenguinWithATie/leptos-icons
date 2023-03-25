#[cfg(feature = "ImPodcast")]
use leptos::*;
#[cfg(feature = "ImPodcast")]
///This icon requires the feature `ImPodcast` to be enabled.
#[component]
pub fn Podcast(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M16 8c0-4.418-3.582-8-8-8s-8 3.582-8 8c0 3.438 2.169 6.37 5.214 7.501l-0.214 0.499h6l-0.214-0.499c3.045-1.131 5.214-4.063 5.214-7.501zM7.606 9.919c-0.356-0.153-0.606-0.507-0.606-0.919 0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.412-0.25 0.766-0.606 0.919l-0.394-0.919-0.394 0.919zM8.41 9.958c0.908-0.189 1.59-0.994 1.59-1.958 0-1.105-0.895-2-2-2s-2 0.895-2 2c0 0.964 0.682 1.768 1.59 1.957l-1.166 2.721c-1.425-0.612-2.424-2.028-2.424-3.677 0-2.209 1.791-4.188 4-4.188s4 1.978 4 4.188c0 1.649-0.999 3.066-2.424 3.677l-1.166-2.72zM10.757 15.433l-1.155-2.695c1.976-0.668 3.398-2.537 3.398-4.738 0-2.761-2.239-5-5-5s-5 2.239-5 5c0 2.201 1.422 4.070 3.398 4.738l-1.155 2.695c-2.494-1.070-4.24-3.547-4.24-6.433 0-3.865 3.133-7.185 6.997-7.185s6.997 3.32 6.997 7.185c0 2.886-1.747 5.363-4.24 6.433z"
        /> < title > { title } < / title > < / svg >
    }
}
