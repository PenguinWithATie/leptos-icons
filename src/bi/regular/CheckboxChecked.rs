#[cfg(feature = "BiRegularCheckboxChecked")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCheckboxChecked")]
/// *This icon requires the feature* `BiRegularCheckboxChecked` *to be enabled*.
#[component]
pub fn CheckboxChecked(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 5c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h10c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2H7zm0 12V7h10l.002 10H7z" /><path d="M10.996 12.556 9.7 11.285l-1.4 1.43 2.704 2.647 4.699-4.651-1.406-1.422z" /></svg>
   }
}