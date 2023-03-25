#[cfg(feature = "SiLlvm")]
use leptos::*;
#[cfg(feature = "SiLlvm")]
///This icon requires the feature `SiLlvm` to be enabled.
#[component]
pub fn Llvm(
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
        "M20.83 2.978l-.086.095a16.245 16.245 0 0 0-1.19 1.487 5.773 5.773 0 0 0-.446.719 2.88 2.88 0 0 0-.249.596.497.497 0 0 0-.033.177v.004a.297.297 0 0 0-.21.29.3.3 0 0 0 .187.284c.038.371.08 1.142.07 2.2l-.004.142a8.001 8.001 0 0 1-.434 2.327c-.016-.01-.03-.014-.04-.013-.03.003-.11.12-.19.251-.058.09-.119.195-.154.291a.37.37 0 0 0-.03.14.249.249 0 0 0 .01.088c.01.037.022.058.022.058a6.836 6.836 0 0 1-.003.007.144.144 0 0 0-.028.067l-.012.029a6.836 6.836 0 0 1-.257.527l-.077.142a6.836 6.836 0 0 1-.07.115.157.157 0 0 0-.022-.014.16.16 0 0 0-.113.024c-.047.035-.151.194-.498.368a1.384 1.384 0 0 1-.32.114 14.1 14.1 0 0 0 .248-2.07 5.072 5.072 0 0 0-.064-1.057c.003-.01.003-.017.003-.017l-.005.01a4.345 4.345 0 0 0-.104-.458 3.554 3.554 0 0 0-.724-1.385c-.631-.754-1.496-1.14-2.256-1.165l-.125-.002c0-.008 0-.015.002-.022.02-.098.024-.127-.087-.032a.872.872 0 0 0-.057.054 2.986 2.986 0 0 0-.146.007.716.716 0 0 1-.003-.108c.004-.076.024-.127-.087-.032a.553.553 0 0 0-.142.167c-.05.008-.08.015-.08.015l.045-.002-.047.008c-.016-.042-.053-.105-.048-.16.01-.118.043-.14-.037-.065-.098.09-.202.226-.22.3l-.096.03c-.018-.019-.074-.07-.086-.179-.013-.11-.012-.166-.08-.05a1.14 1.14 0 0 0-.13.348c-.032.015-.063.03-.093.046a.27.27 0 0 1-.057-.173c-.005-.144.04-.26-.067-.068a1.634 1.634 0 0 0-.146.406 1.6 1.6 0 0 0-.105.08 2.7 2.7 0 0 1-.008-.17c-.002-.143.021-.296-.067-.026a1.832 1.832 0 0 0-.07.321c-.404.385-.65.917-.644 1.572v.007a3.42 3.42 0 0 0 .014.33c.02.238.057.438.107.605-.095.13-.103.689-.01.818.073.104.496.35.586.399a.367.367 0 0 1 .094.093c.115.188.115.567.027.762-.107.238.107.285.285.32.178.036.415-.023.463-.118.047-.095-.154-.31-.237-.748-.084-.439.13-.474.13-.474s.095.024.237.249c.11.175.227.313.336.413a.727.727 0 0 1-.201.172c.12.005.24-.051.304-.087a.77.77 0 0 0 .07.045 1.015 1.015 0 0 1-.05.05.506.506 0 0 0 .109-.022.295.295 0 0 0 .084.022c.155.012.57-.368.57-.439 0-.018-.015-.04-.038-.063-.042-.045-.12-.1-.207-.177a1.03 1.03 0 0 1-.11-.116.723.723 0 0 1-.141-.374v-.01l-.003-.031s.007-.007.016-.02v.004l.004-.01c.052-.08.15-.379-.47-1.078a3.045 3.045 0 0 0-.924-.706c-.042-.472.15-.913.626-1.13l-.005.005.008-.006c.227-.102.517-.155.877-.136.69.036 1.22.36 1.59.845.514.712.738 1.78.575 2.856-.068.065-.055.075-.013.077a4.616 4.616 0 0 1-.055.273l-.027.015c-.123.074-.071.073 0 .092l-.006.02a3.98 3.98 0 0 1-.066.193.956.956 0 0 0-.087.03c-.135.053-.08.06-.012.09l.044.023a5.62 5.62 0 0 1-.101.237.746.746 0 0 0-.08.017c-.142.036-.088.05-.024.09a.52.52 0 0 1 .04.027 6.783 6.783 0 0 1-.107.214.76.76 0 0 0-.153.025c-.14.036-.087.05-.023.09.034.02.06.04.08.059a8.021 8.021 0 0 1-.094.165l-.023.036a.59.59 0 0 0-.206.024c-.142.036-.088.05-.024.09a.67.67 0 0 1 .104.079 8.61 8.61 0 0 1-.126.18.514.514 0 0 0-.23-.005c-.144.02-.092.04-.033.086.068.053.1.099.118.118l-.062.084c-.044-.058-.15-.124-.246-.193a.85.85 0 0 0-.29-.13c-.048 0-.2.268-1.017.197-.819-.071-.87-.15-.942-.15a.978.978 0 0 0-.228.059l-.034-.02a11.03 11.03 0 0 1-.972-.674c-.464-.37-.72-.647-.72-.647s.137-.125-.047-.284a3.32 3.32 0 0 0-.338-.277 2.157 2.157 0 0 0-.242-.166 3.57 3.57 0 0 1-.088-.127 11.485 11.485 0 0 1-.363-.628 10.455 10.455 0 0 1-.25-.527c-.16-.36-.27-.682-.346-.964a5.62 5.62 0 0 1-.148-.923 2.257 2.257 0 0 1 .004-.242.411.411 0 0 0 .158-.323.411.411 0 0 0-.358-.407c-.204-.427-.982-1.477-3.881-3.68l-.127-.097.005.004-.036-.027C7.067 6.599 7.644 7.587 7.795 8a.411.411 0 0 0-.027.033 18.69 18.69 0 0 0-.018-.013c.018 0 .025-.005.025-.005s-.672-.546-1.787-1.23c-1.118-.687-1.748-.849-1.88-.877l-.026-.006s.206.19.174.285c-.01.032-.168.022-.421.028a3.948 3.948 0 0 0-.725.07 4.1 4.1 0 0 0-1.567.63C.238 7.785 0 8.401 0 8.401s.55-.768 1.58-1.305c-.014.337-.047 1.97.445 4.12.53 2.32 1.53 4.487 1.614 4.665.004.178.016.272.016.272s.148-1.79.756-3.647c.175.283.758 1.117 2.075 2.096 1.628 1.21 2.37 1.536 2.4 1.548.001.157.005.24.005.24s.005-.1.018-.263a4.69 4.69 0 0 1 .552-.67 3.58 3.58 0 0 1 .54-.435c.032.11.063.204.093.276-.02-.097-.036-.19-.052-.282.107.01.538.066 1.206.393.343.168.609.395.798.595a.167.167 0 0 0-.014.006s.123.112.269.403a2.416 2.416 0 0 0-.73.084 2.913 2.913 0 0 0-.098-.19.484.484 0 0 0-.039-.134c-.027-.061-.095-.1-.164-.14a.397.397 0 0 0-.104-.073.225.225 0 0 1-.055-.073c-.084-.184-.798.012-.942.18-.175.204-.209.464-.14.608.021.045.07.082.138.11.002.22.137.75.404 1.133a4.964 4.964 0 0 1-.108-.666c-.009-.16 0-.274.015-.35.047.328.174.684.577 1.124-.248-.54-.22-.861-.195-1.143l.01-.044.005-.001c.036.263.142.534.482.963a.412.412 0 0 1-.014-.028l.014.021a1.626 1.626 0 0 1-.167-1.03l.008-.002c.046.03.091.07.134.126.128.172.188.349.212.47a.528.528 0 0 0 .067.383 18.33 18.33 0 0 1-.72.501c-.23.15-.49.313-.757.47-.14.08-.279.158-.414.23l-.135-.077h.002l-.008-.004c-.97-.559-1.69-.932-2.518-1.082-.93-.166-1.786.283-1.982 1.082a1.284 1.284 0 0 0 .552 1.38l.012.008c.013.01.026.017.04.026l.06.036h.002a2.1 2.1 0 0 0 .69.237l.028.005h.006a3.5 3.5 0 0 0 .412.044c.975.047 2.156-.408 3.323-.982.1.054.202.108.306.166 2.547 1.403 4.098 1.585 6.154.728-.502.151-1.208.356-2.094.344a.05.05 0 0 0-.02-.006v.006a5.46 5.46 0 0 1-.647-.05l-.03-.004v-.003a.267.267 0 0 0-.001.002 6.25 6.25 0 0 1-1.556-.436 7.792 7.792 0 0 1-.745-.362 57.065 57.065 0 0 1-1.006-.569c.348-.178.692-.363 1.026-.545.355-.189.672-.362.918-.5l.009-.004h.007s.083-.019.16-.043c.17-.052.41-.149.695-.283.028.117.095.215.195.253.235.091.407.027.522-.102l.018-.01.047-.037c.05-.03.107-.067.156-.09a1.14 1.14 0 0 1 .053-.022c.04-.01.078-.023.116-.037l.033-.01c-.145.297-.091.581-.091.581.043-.37.22-.608.392-.715.064-.027.13-.05.2-.067.064 0 .116.023.144.07.07.12.217.194.364.226.024.15-.045.404-.4.866l.09-.062.004-.003.072-.053c.143-.108.252-.21.334-.305a.97.97 0 0 0 .19-.305c.013.2-.042.507-.31 1.036 0 0 .317-.276.514-.63.006.191-.01.443-.063.785.345-.673.325-1.085.239-1.326l-.01-.036a.433.433 0 0 0-.01-.065l.042-.057c.118-.166-.048-.344-.154-.414-.06-.04-.2-.154-.454-.157-.225-.27-.51-.15-.958.145a1.297 1.297 0 0 1-.166.092.725.725 0 0 0-.055-.116c.16-.096.326-.2.494-.308.317-.207.553-.416.728-.615.51-.517.69-1.02 1.21-1.446h-.013l.06-.031c.854-.458 1.306-.547 1.338-.553l-.003.128.018-.125a1.795 1.795 0 0 1 .481.344 2.12 2.12 0 0 1 .362.487l.044.3s0-.116-.01-.33c.153-.151 1.02-1.043 1.724-2.466.694-1.404.944-2.523 1.017-2.924a18.188 18.188 0 0 1 .665 2.466s-.002-.133-.03-.374c.096-.336.515-1.872.626-3.756.1-1.68-.065-2.78-.14-3.173.648.536.995 1.306.995 1.306s-.04-.163-.188-.437l.02.027s-.236-.455-.644-.904a6.45 6.45 0 0 0-.157-.172c-.432-.46-.854-.617-1.17-.672a1.57 1.57 0 0 0-.308-.03h-.019c-.125-.003-.2.002-.207-.026-.02-.095.11-.285.11-.285s-.177.046-.554.33c-.23.167-.52.41-.853.76a13.689 13.689 0 0 0-.89.994l-.075.095a.203.203 0 0 0-.023-.017c.076-.33.362-1.117 1.788-3.111zM11.92 14.09a.351.351 0 0 1 .012.006zm-.263 2.606c.088.044.19.106.303.194h-.005a.484.484 0 0 0-.155.07.956.956 0 0 1-.126-.224zm-4.855 1.57c.132 0 .276.013.432.047l.02.005.017.004c.11.027.223.062.337.102l.057.021c.191.074.395.156.611.248.32.14.62.277.838.375l.183.088c-.22.106-.424.196-.577.261-1.014.416-1.83.536-2.338.391-.944-.269-.76-1.536.42-1.542z"
        /> < title > { title } < / title > < / svg >
    }
}
