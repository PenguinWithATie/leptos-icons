#[cfg(feature = "BiRegularCalendarMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCalendarMinus")]
/// *This icon requires the feature* `BiRegularCalendarMinus` *to be enabled*.
#[component]
pub fn CalendarMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 13h8v2H8z" /><path d="M19 4h-2V2h-2v2H9V2H7v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zm.002 16H5V8h14l.002 12z" /></svg>
   }
}