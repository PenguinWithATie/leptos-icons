#[cfg(feature = "SiExercism")]
use leptos::*;
#[cfg(feature = "SiExercism")]
///This icon requires the feature `SiExercism` to be enabled.
#[component]
pub fn Exercism(
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
        "M4.35 1.66c-.959 0-1.686.298-2.181.893-.496.595-.744 1.464-.744 2.605v3.474c0 .805-.086 1.424-.259 1.858-.173.434-.493.84-.96 1.218-.138.113-.206.206-.206.278 0 .072.068.165.205.277.476.386.798.794.967 1.225.17.43.253 1.047.253 1.851v3.462c0 1.15.25 2.023.75 2.618.5.595 1.224.892 2.174.892h.882c.379 0 .618-.018.72-.054.1-.036.15-.127.15-.271 0-.201-.133-.334-.399-.398l-.483-.109c-1.063-.241-1.595-1.29-1.595-3.148v-3.293c0-1.247-.302-2.127-.906-2.642l-.23-.193c-.112-.096-.168-.169-.168-.217 0-.056.056-.129.169-.217l.23-.193c.603-.515.905-1.395.905-2.642V5.641c0-1.11.135-1.88.405-2.31.27-.43.832-.762 1.685-.995.258-.073.387-.19.387-.35 0-.217-.29-.326-.87-.326zm14.419.029c-.58 0-.87.108-.87.325 0 .161.128.278.386.35.854.233 1.416.565 1.686.995.27.43.405 1.2.405 2.31v3.294c0 1.246.302 2.126.906 2.641l.229.193c.113.089.17.161.17.217 0 .049-.057.121-.17.217l-.23.193c-.603.515-.905 1.396-.905 2.642v3.293c0 1.858-.532 2.907-1.595 3.149l-.484.108c-.266.064-.398.197-.398.398 0 .145.05.235.15.272.102.036.341.054.72.054h.882c.95 0 1.675-.298 2.174-.893.5-.595.75-1.467.75-2.617v-3.462c0-.805.084-1.422.253-1.852.17-.43.491-.838.967-1.224.137-.113.205-.205.205-.278 0-.072-.068-.165-.205-.277-.468-.378-.788-.784-.961-1.218-.173-.435-.26-1.054-.26-1.858V5.187c0-1.142-.247-2.01-.743-2.606-.495-.595-1.222-.892-2.18-.892zM7.683 9.735c-1.456 0-2.64 1.111-2.64 2.478h1.02c0-.838.727-1.52 1.62-1.52.892 0 1.619.682 1.619 1.52h1.02c0-1.367-1.183-2.478-2.64-2.478zm8.406 0c-1.456 0-2.639 1.111-2.639 2.478h1.02c0-.838.727-1.52 1.62-1.52.892 0 1.62.682 1.62 1.52h1.02c0-1.367-1.185-2.478-2.64-2.478zM9.71 14.36v.561c0 1.277 1.062 2.316 2.366 2.316 1.305 0 2.367-1.039 2.367-2.316v-.56h-.934v.56c0 .877-.76 1.426-1.433 1.426s-1.48-.273-1.48-1.426v-.56z"
        /> < title > { title } < / title > < / svg >
    }
}
