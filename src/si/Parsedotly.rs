#[cfg(feature = "SiParsedotly")]
use leptos::*;
#[cfg(feature = "SiParsedotly")]
///This icon requires the feature `SiParsedotly` to be enabled.
#[component]
pub fn Parsedotly(
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
        "M12.01 0a.459.459 0 0 0-.29.1s-.724.561-1.193.967c-.47.406-.922.886-.922.886a.926.926 0 0 0-.271.615l.108 3.344c0 .2-.157.205-.226 0l-.868-2.106c-.06-.187-.21-.214-.334-.054a12.945 12.945 0 0 0-1.121 1.573 12.61 12.61 0 0 0-.777 1.717.76.76 0 0 0 .1.624l1.762 2.341c.124.16.093.363-.082.253l-1.844-.994c-.17-.11-.312-.033-.307.163a6.85 6.85 0 0 0 .307 1.898 7.581 7.581 0 0 0 2.034 2.91 14.342 14.342 0 0 1 2.83 4.159 3.544 3.544 0 0 1 2.16 0 14.478 14.478 0 0 1 2.811-4.15 7.69 7.69 0 0 0 1.926-2.63c.252-.7.39-1.435.415-2.178v-.01c0-.195-.132-.271-.307-.162l-1.835.994c-.17.11-.215-.093-.09-.253l1.762-2.332a.714.714 0 0 0 .127-.298.762.762 0 0 0-.018-.335c-.23-.626-.504-1.23-.823-1.817a13.322 13.322 0 0 0-1.075-1.482c-.125-.155-.27-.124-.335.063l-.859 2.124c-.064.187-.23.183-.226 0l.1-3.335a.944.944 0 0 0-.253-.642s-.513-.512-.922-.886A25.168 25.168 0 0 0 12.299.1a.459.459 0 0 0-.29-.1zM4.108 14.77c-.129 0-.173.081-.1.182l.778 1.41c.073.105.002.2-.109.117L2.98 15.114a1.399 1.399 0 0 0-.524-.208 1.385 1.385 0 0 0-.56.028c-.378.059-.923.171-.923.171a.235.235 0 0 0-.144.09.258.258 0 0 0-.055.172s.045.549.09.913a1.338 1.338 0 0 0 .462.977l1.771 1.256c.11.073.03.181-.09.136l-1.582-.353c-.12-.045-.181-.001-.144.136 0 0 .258.714.488 1.184.195.368.413.721.66 1.058a.523.523 0 0 0 .162.126.535.535 0 0 0 .2.063l1.906.046c.125 0 .152.144.037.18l-1.22.452c-.12.037-.13.13-.028.208.357.253.75.465 1.157.624.72.231 1.484.289 2.233.18a8.986 8.986 0 0 0 1.636-.379 3.669 3.669 0 0 1-.036-.46 3.51 3.51 0 0 1 .307-1.42c.2-.446.494-.838.859-1.166h-.018a.558.558 0 0 0-.027-.19 4.887 4.887 0 0 0-.796-2.07 5.275 5.275 0 0 0-.913-.913c-.1-.077-.194-.042-.199.081l-.09 1.284c0 .123-.144.128-.18 0l-.57-1.799a.5.5 0 0 0-.29-.29 9.056 9.056 0 0 0-1.166-.315c-.524-.092-1.283-.145-1.283-.145zm15.783.028s-.76.044-1.284.136a9.184 9.184 0 0 0-1.175.316.523.523 0 0 0-.172.117.521.521 0 0 0-.117.172l-.57 1.8c-.037.118-.176.113-.18 0l-.091-1.285c0-.123-.089-.158-.19-.08a5.34 5.34 0 0 0-.922.912 4.829 4.829 0 0 0-.795 2.052v.19a3.469 3.469 0 0 1 1.157 2.594 3.585 3.585 0 0 1-.027.452 9.08 9.08 0 0 0 1.636.389 4.983 4.983 0 0 0 2.233-.19c.407-.16.791-.362 1.148-.615.1-.077.092-.171-.028-.208l-1.22-.452c-.115-.036-.079-.176.045-.18l1.899-.046a.549.549 0 0 0 .37-.198c.227-.328.425-.667.606-1.022.235-.456.497-1.184.497-1.184.037-.119-.034-.181-.154-.136l-1.573.353c-.124.046-.196-.063-.09-.136l1.78-1.256a1.338 1.338 0 0 0 .461-.976c.047-.379.091-.913.091-.913l-.009-.018a.258.258 0 0 0-.054-.163.248.248 0 0 0-.145-.1s-.544-.108-.922-.162a1.4 1.4 0 0 0-.56-.027c-.186.03-.366.097-.524.198l-1.7 1.374c-.101.082-.182-.003-.108-.126l.777-1.41c.078-.1.039-.177-.09-.172zm-7.883 4.646c-.455 0-.905.13-1.283.38a2.28 2.28 0 0 0-.85 1.021 2.268 2.268 0 0 0 .506 2.486 2.319 2.319 0 0 0 2.504.497 2.294 2.294 0 0 0 1.42-2.106c0-.604-.238-1.19-.67-1.618a2.305 2.305 0 0 0-1.627-.66z"
        /> < title > { title } < / title > < / svg >
    }
}
