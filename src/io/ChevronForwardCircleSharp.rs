#[cfg(feature = "IoChevronForwardCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronForwardCircleSharp")]
/// *This icon requires the feature* `IoChevronForwardCircleSharp` *to be enabled*.
#[component]
pub fn ChevronForwardCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48ZM216,374.63,193.37,352l96-96-96-96L216,137.37,334.63,256Z" /></svg>
   }
}