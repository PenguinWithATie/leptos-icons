#[cfg(feature = "FaBrandsJsfiddle")]
use leptos::*;
#[cfg(feature = "FaBrandsJsfiddle")]
///This icon requires the feature `FaBrandsJsfiddle` to be enabled.
#[component]
pub fn Jsfiddle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M510.634 237.462c-4.727-2.621-5.664-5.748-6.381-10.776-2.352-16.488-3.539-33.619-9.097-49.095-35.895-99.957-153.99-143.386-246.849-91.646-27.37 15.25-48.971 36.369-65.493 63.903-3.184-1.508-5.458-2.71-7.824-3.686-30.102-12.421-59.049-10.121-85.331 9.167-25.531 18.737-36.422 44.548-32.676 76.408.355 3.025-1.967 7.621-4.514 9.545-39.712 29.992-56.031 78.065-41.902 124.615 13.831 45.569 57.514 79.796 105.608 81.433 30.291 1.031 60.637.546 90.959.539 84.041-.021 168.09.531 252.12-.48 52.664-.634 96.108-36.873 108.212-87.293 11.54-48.074-11.144-97.3-56.832-122.634zm21.107 156.88c-18.23 22.432-42.343 35.253-71.28 35.65-56.874.781-113.767.23-170.652.23 0 .7-163.028.159-163.728.154-43.861-.332-76.739-19.766-95.175-59.995-18.902-41.245-4.004-90.848 34.186-116.106 9.182-6.073 12.505-11.566 10.096-23.136-5.49-26.361 4.453-47.956 26.42-62.981 22.987-15.723 47.422-16.146 72.034-3.083 10.269 5.45 14.607 11.564 22.198-2.527 14.222-26.399 34.557-46.727 60.671-61.294 97.46-54.366 228.37 7.568 230.24 132.697.122 8.15 2.412 12.428 9.848 15.894 57.56 26.829 74.456 96.122 35.142 144.497zm-87.789-80.499c-5.848 31.157-34.622 55.096-66.666 55.095-16.953-.001-32.058-6.545-44.079-17.705-27.697-25.713-71.141-74.98-95.937-93.387-20.056-14.888-41.99-12.333-60.272 3.782-49.996 44.071 15.859 121.775 67.063 77.188 4.548-3.96 7.84-9.543 12.744-12.844 8.184-5.509 20.766-.884 13.168 10.622-17.358 26.284-49.33 38.197-78.863 29.301-28.897-8.704-48.84-35.968-48.626-70.179 1.225-22.485 12.364-43.06 35.414-55.965 22.575-12.638 46.369-13.146 66.991 2.474C295.68 280.7 320.467 323.97 352.185 343.47c24.558 15.099 54.254 7.363 68.823-17.506 28.83-49.209-34.592-105.016-78.868-63.46-3.989 3.744-6.917 8.932-11.41 11.72-10.975 6.811-17.333-4.113-12.809-10.353 20.703-28.554 50.464-40.44 83.271-28.214 31.429 11.714 49.108 44.366 42.76 78.186z"
        /> < title > { title } < / title > < / svg >
    }
}
