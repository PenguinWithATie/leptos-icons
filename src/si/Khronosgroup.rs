#[cfg(feature = "SiKhronosgroup")]
use leptos::*;
#[cfg(feature = "SiKhronosgroup")]
///This icon requires the feature `SiKhronosgroup` to be enabled.
#[component]
pub fn Khronosgroup(
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
        "M.511 12.946H0v-2.615h.511v2.615zm.008-1.299l.791-1.316h.571l-.839 1.298.839 1.316h-.57l-.792-1.298zm3.11.238v1.061h-.511v-2.615h.511v1.061h.843v-1.061h.511v2.615h-.51v-1.061h-.844zm3.37 1.061h-.511v-2.615h.621c.198 0 .345.006.442.021a.787.787 0 0 1 .587.358.82.82 0 0 1 .117.439.806.806 0 0 1-.167.515.743.743 0 0 1-.453.269l.653 1.014h-.609l-.678-1.212-.002 1.211zm0-1.34h.114c.208 0 .365-.037.474-.112a.377.377 0 0 0 .161-.328c0-.139-.045-.241-.136-.307-.09-.066-.231-.099-.422-.099h-.191v.846zm8.51 1.34h-.502v-2.615h.485L16.574 12v-1.669h.502v2.615h-.475l-1.092-1.72v1.72zm2.962-1.313a1.353 1.353 0 0 1 .228-.765c.129-.189.292-.333.487-.435a1.358 1.358 0 0 1 1.141-.053 1.37 1.37 0 0 1 .731.729 1.413 1.413 0 0 1 .005 1.04c-.065.164-.159.31-.283.436a1.327 1.327 0 0 1-.963.413c-.195 0-.374-.035-.54-.105a1.27 1.27 0 0 1-.436-.309 1.34 1.34 0 0 1-.37-.951zm.535-.018a.854.854 0 0 0 .221.58.83.83 0 0 0 .274.201.765.765 0 0 0 .316.071.762.762 0 0 0 .576-.244.847.847 0 0 0 .232-.608.78.78 0 0 0-.235-.569.764.764 0 0 0-.564-.236.799.799 0 0 0-.582.235.775.775 0 0 0-.238.57zm3.389.522l.469-.001v.037c0 .11.028.198.085.263a.286.286 0 0 0 .227.099.32.32 0 0 0 .241-.092.347.347 0 0 0 .089-.25c0-.168-.12-.289-.36-.362a1.981 1.981 0 0 1-.074-.023c-.203-.063-.354-.154-.452-.27a.665.665 0 0 1-.147-.443c0-.245.068-.443.204-.591a.702.702 0 0 1 .544-.223c.212 0 .381.066.506.198s.196.318.21.555h-.464v-.022a.271.271 0 0 0-.076-.2.268.268 0 0 0-.198-.076c-.084 0-.15.026-.199.079s-.072.124-.072.214c0 .039.005.072.015.101.01.03.026.056.048.079.049.054.155.103.316.15.072.019.127.035.165.049.179.061.312.148.398.262s.13.259.13.436c0 .27-.075.485-.225.649a.768.768 0 0 1-.595.245.733.733 0 0 1-.566-.236c-.141-.16-.215-.369-.219-.627zm-5.032 1.337l-.07.077a.334.334 0 0 0-.103-.065.292.292 0 0 0-.111-.021.26.26 0 0 0-.2.084c-.054.056-.08.124-.08.205s.027.15.08.205c.054.056.12.084.2.084a.29.29 0 0 0 .189-.067v-.173h-.155v-.102h.266v.336a.619.619 0 0 1-.147.085.436.436 0 0 1-.152.028.383.383 0 0 1-.395-.395.4.4 0 0 1 .233-.363.428.428 0 0 1 .32-.002.332.332 0 0 1 .125.084zm1.091-.094h.143a.83.83 0 0 1 .099.006.247.247 0 0 1 .068.022c.032.017.058.043.076.076s.028.072.028.114a.182.182 0 0 1-.048.128.213.213 0 0 1-.128.067l.229.31-.106.045-.252-.365v.347l-.112.009.003-.759zm.113.1v.238h.04c.048 0 .085-.01.112-.032a.112.112 0 0 0 .04-.09.106.106 0 0 0-.037-.087.181.181 0 0 0-.111-.03h-.044v.001zm1.738-.119a.377.377 0 0 1 .256.095.403.403 0 0 1-.097.665.39.39 0 0 1-.161.032.395.395 0 0 1-.257-.095.404.404 0 0 1-.138-.302.4.4 0 0 1 .254-.371.402.402 0 0 1 .143-.024zm0 .104a.26.26 0 0 0-.2.084c-.054.056-.08.124-.08.205s.027.15.08.205c.054.056.12.083.201.083a.284.284 0 0 0 .281-.289.279.279 0 0 0-.282-.288zm1.563-.095v.426c0 .09.012.154.035.191.023.036.062.054.117.054s.096-.017.12-.052.037-.092.037-.172v-.436l.108-.01v.473c0 .061-.004.108-.013.141s-.025.061-.045.085a.216.216 0 0 1-.087.059.318.318 0 0 1-.115.022c-.09 0-.159-.026-.203-.076-.045-.05-.067-.128-.067-.231v-.461l.113-.013zm1.515.01h.199c.085 0 .151.019.196.058.045.039.067.096.067.169 0 .07-.026.125-.077.167a.335.335 0 0 1-.213.061.832.832 0 0 1-.057-.003h-.003v.297l-.112.009v-.758zm.112.1v.25l.031.004a.433.433 0 0 0 .037.001c.054 0 .096-.01.124-.032a.114.114 0 0 0 .043-.096c0-.044-.013-.076-.039-.097-.026-.021-.067-.031-.123-.031h-.073v.001zm-12.03-.704c-.897-.032-2.123-.442-2.377-1.033-.219-.538-.001-1.165 1.034-1.61.615-.256 1.544-.328 2.261-.262 1.016.093 1.985.618 1.986.621v.737l-.001.049s-.315-.285-.709-.515c-.371-.217-.71-.369-1.384-.431-.158-.014-.426-.05-.861.01-.176.025-.423.043-.981.288a2.342 2.342 0 0 0-.476.285 2.953 2.953 0 0 0-.223.201c-.159.199-.243.349-.203.62.046.204.137.332.399.527.114.085.127.084.177.11.469.243.843.332 1.378.399m.03-2.294c.9-.004 2.176.356 2.507.937.291.528.156 1.162-.817 1.646-.578.28-1.496.389-2.219.351-1.026-.053-2.062-.54-2.063-.541l-.098-.735-.006-.049s.352.272.776.487c.399.201.758.341 1.438.376.159.009.431.032.857-.044.172-.031.417-.059.94-.325a1.98 1.98 0 0 0 .632-.512c.132-.205.196-.358.12-.626-.074-.201-.181-.325-.467-.51-.124-.08-.137-.079-.191-.103-.5-.223-.884-.297-1.428-.343"
        /> < title > { title } < / title > < / svg >
    }
}
