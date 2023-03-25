#[cfg(feature = "SiDior")]
use leptos::*;
#[cfg(feature = "SiDior")]
///This icon requires the feature `SiDior` to be enabled.
#[component]
pub fn Dior(
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
        "M.0728 8.7751h3.1157c2.6789 0 3.7272 1.5724 3.7272 3.237 0 1.6937-1.3443 3.203-3.9019 3.203H.0776c-.0534 0-.0728-.034-.0728-.0631 0-.034.034-.0631.0825-.0631h.5484c.1699 0 .2815-.1019.2815-.2912V9.2022c0-.1407-.0679-.296-.2912-.296H.0679C.0243 8.9062 0 8.8771 0 8.8431c0-.0291.0097-.068.0728-.068m1.9461 6.1392c0 .1407.0631.1844.1553.1844h.825c2.0334 0 2.7711-1.5578 2.7711-3.1205S4.9889 8.9013 3.271 8.9013H2.1499c-.1165 0-.1262.0971-.1262.1407l-.0048 5.8723zm5.2073-6.1392c-.0485 0-.0922.0194-.0922.0582s.0243.0631.0679.0631h.5484c.1262 0 .2475.0874.2475.33v5.5762c0 .1165-.0874.2912-.2427.2912h-.5434c-.0631 0-.0679.0485-.0679.0679s-.0049.0534.0679.0534h2.6401c.0388 0 .0922-.0049.0922-.0437s-.0097-.0776-.0776-.0776h-.5097c-.0728 0-.2718-.0437-.2718-.2669V9.1682c0-.1602.1019-.2669.2863-.2669h.4999c.0437 0 .0679-.0243.0679-.0582 0-.034-.0243-.0631-.0825-.0631-.0005-.0048-2.6396.0092-2.6303-.0049zm4.1251 3.2225c0-1.7811.7959-3.2224 2.2906-3.2224 1.4656 0 2.2906 1.4414 2.2906 3.2224S15.1804 15.22 13.642 15.22c-1.4899.0049-2.2907-1.4414-2.2907-3.2224m2.2907 3.3583c2.1256 0 3.4651-1.5044 3.4651-3.3535s-1.3249-3.3583-3.4651-3.3583c-2.1353 0-3.4651 1.5044-3.4651 3.3535s1.3831 3.3583 3.4651 3.3583m10.2593-.1796c-.8347.0874-1.2958-1.2812-1.718-1.9315-.3154-.4805-.9852-.9706-1.6452-1.0677 1.0871-.0631 2.3052-.4125 2.3052-1.6452 0-.9997-.6163-1.7568-2.8779-1.7568h-2.6061c-.034 0-.0679.0194-.0679.0582s.034.0631.0679.0631h.5969c.1262 0 .2475.0874.2475.33v5.5762c0 .1165-.0874.2912-.2427.2912h-.587c-.0485 0-.0679.0388-.0679.0582s.0194.0631.0679.0631h2.7662c.0388 0 .0728-.0194.0728-.0582s-.0243-.0631-.0776-.0631h-.5581c-.0728 0-.2718-.0485-.2718-.2669v-2.6352h.2863c1.3686 0 1.4705 1.485 2.1499 2.3343.5824.728 1.3443.8202 1.7762.8202.1844 0 .3106-.0049.4271-.034.0728-.0243.0874-.1504-.0437-.1359m-4.3192-6.2798h.4028c.6891 0 1.8053.2718 1.8053 1.5724 0 1.1939-.99 1.5967-1.9073 1.5967h-.5872V9.1682c.0001-.165.102-.2717.2864-.2717"
        /> < title > { title } < / title > < / svg >
    }
}
