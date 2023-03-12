#[cfg(feature = "BiSolidCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCard")]
/// *This icon requires the feature* `BiSolidCard` *to be enabled*.
#[component]
pub fn Card(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 17c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H6c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h12zM4 19h16v2H4z" /></svg>
   }
}