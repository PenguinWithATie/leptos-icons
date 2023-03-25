#[cfg(feature = "SiDaimler")]
use leptos::*;
#[cfg(feature = "SiDaimler")]
///This icon requires the feature `SiDaimler` to be enabled.
#[component]
pub fn Daimler(
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
        "M.718 10.48c-.622 0-.64.039-.609.225h.207c.195 0 .238.026.238.194v2.038c0 .29-.082.32-.544.346a.368.368 0 0 0 0 .168h1.623c1.235 0 1.745-.596 1.745-1.486 0-.924-.54-1.485-1.784-1.485zm4.573.03c-.203 0-.26.07-.402.402l-.855 2.025c-.116.276-.155.302-.488.346a.369.369 0 0 0 0 .168h1.235c.108 0 .138-.07.108-.194h-.371c-.238 0-.238-.065-.138-.32l.177-.47H5.67l.272.651c.047.113 0 .152-.47.165a.367.367 0 0 0 0 .168H6.87c.113 0 .139-.07.113-.194h-.1c-.22 0-.267-.04-.315-.152l-.997-2.275c-.07-.151-.1-.225-.113-.32zm16.374.026c-.604 0-.863 0-1.07.017-.178.013-.208.095-.178.208h.207c.178 0 .238.026.238.194v1.995c0 .263-.087.307-.462.333a.442.442 0 0 0 0 .168h1.36c.125 0 .138-.07.125-.194h-.22c-.224 0-.263-.044-.263-.169v-.635h.289c.263 0 .272.013.38.208.112.198.268.371.41.496.32.277.583.363.985.363.458 0 .566-.112.527-.263-.376 0-.566-.04-1.136-.497a15.656 15.656 0 0 1-.488-.432c.358-.15.583-.445.583-.807 0-.592-.458-.985-1.287-.985zm-14.21.03c-.182 0-.208.082-.182.195h.207c.199 0 .238.025.238.181v2.064c0 .177-.082.22-.428.277a.279.279 0 0 0 0 .168h1.313c.13 0 .142-.07.112-.194h-.233c-.181 0-.225-.03-.225-.182v-2.051c0-.181.056-.263.432-.29a.37.37 0 0 0 0-.168zm2.232 0c-.182 0-.208.095-.182.195.428.013.454.06.44.198l-.176 2.03c-.013.181-.052.268-.527.294a.37.37 0 0 0 0 .168h1.416c.121 0 .138-.07.108-.194h-.233c-.458 0-.458-.044-.445-.212l.138-1.939 1.192 2.358c.1 0 .134-.026.212-.181l1.036-2.177.194 1.883c.022.211-.038.259-.4.28l-.234.014a.281.281 0 0 0 0 .168h1.524c.121 0 .134-.07.121-.194-.333-.044-.402-.057-.415-.195l-.207-2.107c-.013-.125.013-.194.415-.22a.278.278 0 0 0 0-.169H12.64l-.997 2.077-1.084-2.077zm5.017 0c-.181 0-.207.082-.181.195h.207c.182 0 .238.025.238.181v1.978c0 .28-.095.337-.514.363a.444.444 0 0 0 0 .168h2.107a6.16 6.16 0 0 1 .225-.777c.047-.134-.018-.186-.143-.182l-.12.268c-.182.389-.225.41-.545.41h-.276c-.121 0-.194-.039-.194-.181V11.08c0-.28.086-.32.54-.345a.371.371 0 0 0 0-.169zm2.799 0c-.19 0-.208.082-.19.195h.207c.194 0 .233.025.233.181v1.995c0 .29-.082.32-.54.346a.281.281 0 0 0 0 .168h2.384a6.26 6.26 0 0 1 .224-.777c.044-.13-.017-.186-.142-.182l-.12.268c-.182.389-.226.41-.545.41h-.371c-.294 0-.333-.065-.333-.22v-.76h.453c.113 0 .169.039.225.376a.203.203 0 0 0 .15 0 9.378 9.378 0 0 1 0-.851c.01-.134-.038-.164-.124-.164h-.052l-.03.22c-.013.112-.04.156-.169.156h-.453v-1.11H19c.238 0 .329.069.402.332l.057.195c.138.013.19-.06.168-.195a6.266 6.266 0 0 1-.069-.583zm-16.41.169h.445c.83 0 1.261.483 1.261 1.312 0 .929-.47 1.166-1.209 1.166-.401 0-.496-.043-.496-.306zm20.309.052h.147c.488 0 .833.18.833.764 0 .583-.346.652-.773.652h-.207zm-16.301.32l.47 1.122h-.92z"
        /> < title > { title } < / title > < / svg >
    }
}
