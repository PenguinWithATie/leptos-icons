#[cfg(feature = "Tb3dCubeSphereOff")]
use leptos::*;
#[cfg(feature = "Tb3dCubeSphereOff")]
///This icon requires the feature `Tb3dCubeSphereOff` to be enabled.
#[component]
pub fn _3dCubeSphereOff(
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
        "icon icon-tabler icon-tabler-3d-cube-sphere-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 17.6l-2 -1.1v-2.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M4 10v-2.5l2 -1.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 4.1l2 -1.1l2 1.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 6.4l2 1.1v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 14v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 19.9l-2 1.1l-2 -1.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 8.6l2 -1.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 18.5v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12l-2 -1.12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 8.6l-2 -1.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
