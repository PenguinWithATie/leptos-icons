#[cfg(feature = "CgCaptions")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgCaptions")]
/// *This icon requires the feature* `CgCaptions` *to be enabled*.
#[component]
pub fn Captions(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11 8V10H8V14H11V16H6V8H11Z" fill="currentColor" /><path d="M18 8V10H15V14H18V16H13V8H18Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 5C2 4.44772 2.44772 4 3 4H21C21.5523 4 22 4.44772 22 5V19C22 19.5523 21.5523 20 21 20H3C2.44772 20 2 19.5523 2 19V5ZM4 18V6H20V18H4Z" fill="currentColor" /></svg>
   }
}