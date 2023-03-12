#[cfg(feature = "VsPlug")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPlug")]
/// *This icon requires the feature* `VsPlug` *to be enabled*.
#[component]
pub fn Plug(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7 1H6v3H4.5l-.5.5V8a4 4 0 0 0 3.5 3.969V15h1v-3.031A4 4 0 0 0 12 8V4.5l-.5-.5H10V1H9v3H7V1zm3.121 9.121A3 3 0 0 1 5 8V5h6v3a3 3 0 0 1-.879 2.121z" /></svg>
   }
}