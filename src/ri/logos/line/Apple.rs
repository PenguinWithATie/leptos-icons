#[cfg(feature = "RiLogosLineApple")]
use leptos::*;
#[cfg(feature = "RiLogosLineApple")]
///This icon requires the feature `RiLogosLineApple` to be enabled.
#[component]
pub fn Apple(
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
        "M15.729 8.208c-.473-.037-.981.076-1.759.373.066-.025-.742.29-.968.37-.502.175-.915.271-1.378.271-.458 0-.88-.092-1.366-.255-.155-.053-.311-.11-.505-.186-.082-.032-.382-.152-.448-.177-.648-.254-1.013-.35-1.316-.342-1.152.015-2.243.68-2.876 1.782-1.292 2.244-.577 6.299 1.312 9.031 1.006 1.444 1.556 1.96 1.778 1.953.222-.01.385-.057.783-.225l.167-.071c1.005-.429 1.71-.618 2.771-.618 1.021 0 1.703.186 2.668.602l.168.072c.398.17.542.208.792.202.358-.005.799-.417 1.778-1.854.268-.391.505-.803.71-1.22a7.354 7.354 0 0 1-.392-.347c-1.289-1.228-2.086-2.884-2.108-4.93a6.625 6.625 0 0 1 1.41-4.181 4.124 4.124 0 0 0-1.221-.25zm.155-1.994c.708.048 2.736.264 4.056 2.196-.108.06-2.424 1.404-2.4 4.212.036 3.36 2.94 4.476 2.976 4.488-.024.084-.468 1.596-1.536 3.156-.924 1.356-1.884 2.7-3.396 2.724-1.488.036-1.968-.876-3.66-.876-1.704 0-2.232.852-3.636.912-1.464.048-2.568-1.464-3.504-2.808-1.908-2.76-3.36-7.776-1.404-11.172.972-1.692 2.7-2.76 4.584-2.784 1.428-.036 2.784.96 3.66.96.864 0 2.412-1.152 4.26-1.008zm-1.14-1.824c-.78.936-2.052 1.668-3.288 1.572-.168-1.272.456-2.604 1.176-3.432.804-.936 2.148-1.632 3.264-1.68.144 1.296-.372 2.604-1.152 3.54z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
