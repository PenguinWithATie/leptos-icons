#[cfg(feature = "HiLgSolidScissors")]
use leptos::*;
#[cfg(feature = "HiLgSolidScissors")]
///This icon requires the feature `HiLgSolidScissors` to be enabled.
#[component]
pub fn Scissors(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M8.12833 9.15465C6.98189 10.5304 4.97858 10.9236 3.37489 9.99773C1.5813 8.9622 0.966765 6.66874 2.0023 4.87514C3.03783 3.08154 5.3313 2.46701 7.12489 3.50254C8.66319 4.39068 9.33419 6.20412 8.84075 7.83392L9.97714 8.49001C10.1638 8.59775 10.295 8.78056 10.3374 8.99182C10.3799 9.20309 10.3294 9.4224 10.1989 9.59385L10.1933 9.6012C9.94809 9.92324 9.49126 9.99309 9.16144 9.75845C9.12519 9.73266 9.08749 9.70843 9.04842 9.68586L8.12833 9.15465ZM3.30134 5.62514C3.92266 4.54898 5.29874 4.18026 6.37489 4.80158C7.3896 5.38742 7.77536 6.6443 7.29525 7.68806C7.27427 7.71504 7.25489 7.74377 7.23732 7.7742C7.21056 7.82057 7.1893 7.8686 7.17336 7.91755C6.54071 8.96089 5.18679 9.31178 4.12489 8.6987C3.04874 8.07737 2.68002 6.7013 3.30134 5.62514Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M13.3484 8.27167C12.3744 8.56954 11.5401 9.1424 10.9215 9.88959C10.4538 10.4545 10.109 11.1196 9.91984 11.842C9.81853 12.2288 9.76198 12.6318 9.75505 13.0438C9.74669 13.5407 9.47822 13.9968 9.04782 14.2453L8.08883 14.799C6.93631 13.4611 4.96032 13.0872 3.37489 14.0025C1.5813 15.0381 0.966765 17.3315 2.0023 19.1251C3.03783 20.9187 5.3313 21.5333 7.12489 20.4977C8.68172 19.5989 9.35023 17.7523 8.8224 16.1075L22.5507 8.18151C22.8134 8.02983 22.9588 7.73485 22.9193 7.43409C22.8797 7.13334 22.6628 6.88606 22.3698 6.80754L21.5674 6.59253C20.6169 6.33787 19.6141 6.35542 18.6731 6.64319L13.3484 8.27167ZM4.12489 15.3016C3.04874 15.9229 2.68002 17.299 3.30134 18.3751C3.92266 19.4513 5.29874 19.82 6.37489 19.1987C7.45105 18.5774 7.81977 17.2013 7.19845 16.1251C6.57713 15.049 5.20105 14.6803 4.12489 15.3016ZM12 12.75C12.4142 12.75 12.75 12.4142 12.75 12C12.75 11.5858 12.4142 11.25 12 11.25C11.5858 11.25 11.25 11.5858 11.25 12C11.25 12.4142 11.5858 12.75 12 12.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.3718 12.615C16.6038 12.481 16.8897 12.481 17.1218 12.615L22.5513 15.7497C22.814 15.9014 22.9594 16.1963 22.9198 16.4971C22.8803 16.7979 22.6634 17.0451 22.3704 17.1236L21.5679 17.3387C20.6175 17.5933 19.6147 17.5758 18.6737 17.288L13.5271 15.714C13.2415 15.6267 13.035 15.3783 13.0012 15.0816C12.9674 14.785 13.1128 14.4965 13.3714 14.3473L16.3718 12.615Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
