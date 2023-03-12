#[cfg(feature = "SiBuzzfeed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBuzzfeed")]
/// *This icon requires the feature* `SiBuzzfeed` *to be enabled*.
#[component]
pub fn Buzzfeed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 12c0 6.627-5.373 12-12 12S0 18.627 0 12 5.373 0 12 0s12 5.373 12 12zm-4.148-.273l-.977-6.94-6.5 2.624 2.575 1.487-2.435 4.215L8.3 10.68l-4.153 7.19 2.327 1.346 2.812-4.868L13.5 16.78l3.777-6.54 2.575 1.487z" /></svg>
   }
}