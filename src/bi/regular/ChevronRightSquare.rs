#[cfg(feature = "BiRegularChevronRightSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronRightSquare")]
/// *This icon requires the feature* `BiRegularChevronRightSquare` *to be enabled*.
#[component]
pub fn ChevronRightSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zM5 5h14l.001 14H5V5z" /><path d="M9.293 7.707 13.586 12l-4.293 4.293 1.414 1.414L16.414 12l-5.707-5.707z" /></svg>
   }
}