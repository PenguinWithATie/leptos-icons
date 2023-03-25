#[cfg(feature = "SiDm")]
use leptos::*;
#[cfg(feature = "SiDm")]
///This icon requires the feature `SiDm` to be enabled.
#[component]
pub fn Dm(
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
        "M8.1 8.683c-1.237 0-1.943 1.726-1.943 2.95 0 .502.122.72.38.72.693 0 1.876-2.012 2.12-3.262l.068-.326a2.588 2.588 0 0 0-.625-.082m1.468 5.192H7.76c.082-.476.23-.992.49-1.7h-.028c-.57.952-1.372 1.863-2.391 1.863-1.047 0-1.577-.68-1.577-2.093 0-2.487 1.29-4.757 4.172-4.757a3.3 3.3 0 0 1 .57.055l.3-1.51v-.012a9.956 9.956 0 0 0-1.114-.136l.258-1.237c.91-.095 1.848-.163 2.949-.122zm10.83 0h-1.793l.653-4.023c.136-.652.04-.91-.258-.91-.612 0-1.7 1.603-1.957 3.18l-.286 1.753h-1.794l.666-4.023c.136-.652.04-.91-.258-.91-.612 0-1.672 1.59-1.97 3.139l-.34 1.794h-1.794l.965-5.056v-.04c-.286-.055-.72-.11-1.115-.137l.258-1.236a17.857 17.857 0 0 1 2.84-.123 12.31 12.31 0 0 1-.502 1.7h.027c.598-1.02 1.237-1.795 2.392-1.795.775 0 1.29.408 1.29 1.305 0 .163-.04.326-.108.598l.014.014c.598-1.088 1.345-1.903 2.5-1.903 1.278 0 1.55.924 1.305 2.16zm3.562 1.74s-.258.312-.802.665c-1.06.72-3.166 1.63-6.442.816a40.26 40.26 0 0 1-.897-.245c-2.99-.87-4.756-1.998-8.738-2.487-2.705.068-5.45.38-6.958.978 0 0 .407.422.53.598.068.136.203.53.34.938.149.435.285.843.285.843s1.699-.897 3.683-1.427c.938-.258 1.93-.408 2.84-.354 3.044.19 5.504 1.767 8.236 3.045 1.889.53 6.51.937 7.814.353-.013-.027.245-1.291.11-3.724m-.803.666c-1.06.72-3.166 1.63-6.442.816a40.26 40.26 0 0 1-.897-.245c-2.065-.897-4.158-1.848-5-2.487 2.364.367 5.897.734 11.605-.19 0 0 .544.856.734 2.106m-22.166.585L0 17.218s1.142 1.359 1.332 1.794l.286.775s3.6-1.985 8.833-1.686a9.99 9.99 0 0 1 2.23.408s-2.135-1.386-2.556-1.522a26.5 26.5 0 0 0-5.164-.707 19.904 19.904 0 0 0-3.683 1.427s-.15-.421-.286-.842"
        /> < title > { title } < / title > < / svg >
    }
}
