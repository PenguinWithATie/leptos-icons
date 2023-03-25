#[cfg(feature = "SiYamahacorporation")]
use leptos::*;
#[cfg(feature = "SiYamahacorporation")]
///This icon requires the feature `SiYamahacorporation` to be enabled.
#[component]
pub fn Yamahacorporation(
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
        "M12 0a12 12 0 1012 12A12 12 0 0012 0zm-.412 1.377A10.648 10.648 0 0122.664 12 10.648 10.648 0 0112 22.664a10.648 10.648 0 01-.412-21.287zM12 1.9a.184.184 0 00-.172.131l-.883 2.526a.39.39 0 000 .097.3.3 0 00.133.25.926.926 0 01.074 1.465 2.345 2.345 0 00-.82 1.79v.903L5.865 6.498l-.547.953 4.987 2.877v.692l-.602.347-4.978-2.88-.551.954L8.602 12l-.782.45a2.345 2.345 0 00-1.127 1.616.93.93 0 01-1.312.668.293.293 0 00-.277 0 .324.324 0 00-.079.063l-1.742 2.037a.188.188 0 00.176.305l2.633-.493a.36.36 0 00.09-.035.3.3 0 00.152-.238.926.926 0 011.232-.781 2.345 2.345 0 001.954-.184l.78-.451v5.104h1.098v-5.756l.598-.344.598.344v5.756h1.1v-5.123l.78.45a2.345 2.345 0 001.954.184.926.926 0 011.234.782.285.285 0 00.149.238.36.36 0 00.09.035l2.634.492a.184.184 0 00.176-.305l.004.02-1.744-2.037a.39.39 0 00-.075-.063.3.3 0 00-.28 0 .938.938 0 01-.864-.035.93.93 0 01-.434-.633 2.368 2.368 0 00-1.14-1.609l-.782-.45 4.436-2.558-.549-.955-4.98 2.873-.602-.347v-.692l4.985-2.877-.547-.953L13.7 9.062v-.904a2.345 2.345 0 00-.803-1.789.922.922 0 01.079-1.465.309.309 0 00.128-.25.27.27 0 000-.097L12.18 2.03a.184.184 0 00-.18-.13zm.021 5.512a.598.598 0 01.58.598V9.7l-.597.347-.598-.348V8.01a.598.598 0 01.615-.597zm-.017 3.818l.687.391v.781l-.687.391-.688-.39v-.782zm2.299 1.403l1.46.847a.598.598 0 01.223.817v.004a.602.602 0 01-.82.219l-1.465-.844v-.696zm-4.596.004l.602.347v.692l-1.465.844a.598.598 0 11-.598-1.036z"
        /> < title > { title } < / title > < / svg >
    }
}
