#[cfg(feature = "BiRegularMinusBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMinusBack")]
/// *This icon requires the feature* `BiRegularMinusBack` *to be enabled*.
#[component]
pub fn MinusBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M14 3H5c-1.103 0-2 .897-2 2v9c0 1.103.897 2 2 2h3v3c0 1.103.897 2 2 2h9c1.103 0 2-.897 2-2v-9c0-1.103-.897-2-2-2h-3V5c0-1.103-.897-2-2-2zM5 5h9l-.003 9H5V5z" /></svg>
   }
}