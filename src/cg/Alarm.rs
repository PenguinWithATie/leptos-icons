#[cfg(feature = "CgAlarm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgAlarm")]
/// *This icon requires the feature* `CgAlarm` *to be enabled*.
#[component]
pub fn Alarm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5.45887 2L1 6.01478L2.33826 7.50107L6.79713 3.48629L5.45887 2Z" fill="currentColor" /><path d="M11 8H13V12H16V14H11V8Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12ZM5 12C5 8.13401 8.13401 5 12 5C15.866 5 19 8.13401 19 12C19 15.866 15.866 19 12 19C8.13401 19 5 15.866 5 12Z" fill="currentColor" /><path d="M18.5411 2L23 6.01478L21.6617 7.50107L17.2029 3.48629L18.5411 2Z" fill="currentColor" /></svg>
   }
}