#[cfg(feature = "TiSocialDribbbleCircular")]
use leptos::*;
#[cfg(feature = "TiSocialDribbbleCircular")]
///This icon requires the feature `TiSocialDribbbleCircular` to be enabled.
#[component]
pub fn SocialDribbbleCircular(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M12 21c-4.962 0-9-4.037-9-9s4.038-9 9-9 9 4.037 9 9-4.038 9-9 9zm0-16c-3.86 0-7 3.141-7 7s3.14 7 7 7 7-3.141 7-7-3.14-7-7-7zM12 6.5c-3.033 0-5.5 2.468-5.5 5.5s2.467 5.5 5.5 5.5 5.5-2.468 5.5-5.5-2.467-5.5-5.5-5.5zm4.49 5.402c-1.048-.186-2.1-.18-3.103.042-.127-.326-.267-.647-.417-.965.875-.524 1.652-1.221 2.284-2.07.746.785 1.211 1.834 1.236 2.993zm-2-3.646c-.546.748-1.215 1.367-1.975 1.832-.479-.856-1.046-1.663-1.692-2.412.378-.103.767-.176 1.177-.176.921 0 1.776.28 2.49.756zm-4.641-.184c.687.758 1.278 1.59 1.776 2.473-1.238.531-2.622.691-3.998.437.293-1.259 1.118-2.302 2.222-2.91zm-2.349 3.928c.468.064.936.121 1.399.121 1.106 0 2.187-.244 3.185-.683.123.261.238.524.344.793-1.469.526-2.769 1.489-3.728 2.805-.738-.802-1.2-1.862-1.2-3.036zm1.948 3.699c.842-1.189 2.004-2.057 3.318-2.527.314 1.001.518 2.039.596 3.095-.433.138-.884.233-1.362.233-.948 0-1.826-.298-2.552-.801zm4.872.137c-.099-1-.296-1.983-.593-2.937.87-.176 1.778-.172 2.683.001-.256 1.247-1.035 2.296-2.09 2.936z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
