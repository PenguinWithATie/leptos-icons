#[cfg(feature = "SiPicnic")]
use leptos::*;
#[cfg(feature = "SiPicnic")]
///This icon requires the feature `SiPicnic` to be enabled.
#[component]
pub fn Picnic(
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
        "M0 0v24h24V0zm17.492 4.481c.851 0 1.635.279 2.261.749.037.029.068.073.027.158-.054.12-.183.413-.416.757a3.834 3.834 0 0 1-.424.555c-.12.117-.12.103-.27.012a2.294 2.294 0 0 0-1.155-.324c-.934 0-1.712.624-1.712 1.655l.005.1c.066 1.034.888 1.603 1.817 1.547a2.27 2.27 0 0 0 .73-.175c.143-.062.314-.153.455-.242.122-.076.148-.116.27-.007.12.114.252.287.395.502.238.345.398.661.458.776.06.1.022.13-.013.16-.412.355-.89.617-1.44.764-.309.09-.64.138-.988.138a4.73 4.73 0 0 1-.288-.009l-.091-.007c-.864-.058-1.64-.371-2.226-.916-.032-.027-.064-.06-.09-.084-.616-.605-.995-1.468-.995-2.547 0-2.26 1.653-3.562 3.69-3.562zm-5.567.108h.15c.752 0 .785.043.837.082a.116.116 0 0 1 .038.073c.037.207.052 1.437.048 2.658.004 1.221-.011 2.438-.048 2.645 0 0-.002.045-.038.075-.052.041-.085.08-.838.083h-.149c-.754-.003-.788-.042-.833-.083-.043-.03-.044-.075-.044-.075-.035-.207-.051-1.424-.047-2.645-.004-1.221.012-2.451.047-2.658 0 0 .001-.04.044-.073.045-.04.079-.082.833-.082zm-5.388.001c2.328.047 3.63.92 3.657 2.908v.091c0 1.895-1.357 2.909-3.657 2.909a8.94 8.94 0 0 1-.44-.012c-.008.419-.017.738-.03.817 0 0-.007.044-.043.076-.05.039-.083.079-.836.082h-.149c-.754-.003-.786-.043-.835-.082-.04-.032-.042-.076-.042-.076-.048-1.232-.045-2.183-.045-3.346-.002-1.223.009-2.991.045-3.181.017-.094.082-.113.155-.125.307-.038 1.661-.072 2.22-.06zm-.175 1.85a4.57 4.57 0 0 0-.249.014v2.162c.154.012.295.025.587.019.951-.016 1.502-.343 1.513-1.083v-.03C8.203 6.781 7.65 6.455 6.7 6.44a5.676 5.676 0 0 0-.338 0zM11.999 11c.891 0 1.337 1.077.707 1.707-.63.63-1.707.184-1.707-.707 0-.55.451-1 1-1zm5.493 1.392a3.76 3.76 0 0 1 2.261.748c.037.028.068.076.027.161-.054.117-.183.409-.416.757-.135.21-.248.36-.424.554-.12.118-.12.104-.27.012a2.263 2.263 0 0 0-1.155-.323c-.934 0-1.712.622-1.712 1.654l.005.1c.066 1.032.888 1.607 1.817 1.544a2.23 2.23 0 0 0 .73-.173c.143-.06.314-.153.455-.24.122-.077.148-.118.27-.007.12.112.252.287.395.5.238.344.398.66.458.776.06.097.022.129-.013.161-.412.353-.89.613-1.44.764a3.57 3.57 0 0 1-1.276.13l-.091-.007c-.864-.06-1.64-.373-2.226-.916l-.09-.088c-.616-.603-.995-1.465-.995-2.544 0-2.26 1.653-3.563 3.69-3.563zm-12.453.148h.149c.753 0 .786.045.836.085.009.009.02.023.03.031.456.833.806 1.402 1.057 1.826.478.806.674 1.164.9 1.556a89.525 89.525 0 0 1 .048-3.337s.004-.046.042-.076c.051-.04.084-.084.836-.085h.15c.751 0 .781.045.831.085.042.03.043.076.043.076.035.202.05 2.093.048 3.312.002 1.222-.013 3.022-.048 3.228 0 0 0 .043-.043.074-.05.04-.08.084-.831.086h-.15c-.752-.002-.785-.047-.836-.086a.076.076 0 0 1-.026-.029c-.024-.03-1.9-3.123-1.962-3.225.003 1.22-.013 2.976-.047 3.18 0 0-.007.043-.042.074-.05.04-.083.084-.836.086h-.149c-.754-.002-.786-.047-.835-.086-.04-.031-.042-.074-.042-.074-.034-.206-.046-2.01-.045-3.232-.001-1.221.011-3.106.045-3.308 0 0 .001-.046.042-.076.05-.04.081-.084.835-.085zm6.886 1.256h.149c.753 0 .786.041.838.082.021.02.035.045.038.074.037.205.052 1.424.048 2.644.004 1.222-.011 2.439-.048 2.645 0 0-.002.043-.038.074-.052.04-.085.084-.838.086h-.149c-.754-.002-.788-.047-.833-.086a.102.102 0 0 1-.044-.074c-.035-.206-.051-1.423-.047-2.645-.004-1.22.012-2.44.047-2.644 0 0 .001-.042.044-.074.045-.04.079-.082.833-.082Z"
        /> < title > { title } < / title > < / svg >
    }
}
