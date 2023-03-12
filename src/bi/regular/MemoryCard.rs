#[cfg(feature = "BiRegularMemoryCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMemoryCard")]
/// *This icon requires the feature* `BiRegularMemoryCard` *to be enabled*.
#[component]
pub fn MemoryCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 4v16c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V8a.997.997 0 0 0-.293-.707l-5-5A.996.996 0 0 0 14 2H6c-1.103 0-2 .897-2 2zm14 4.414L18.001 20H6V4h7.586L18 8.414z" /><path d="M8 6h2v4H8zm4 0h2v4h-2z" /></svg>
   }
}