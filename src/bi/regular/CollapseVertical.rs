#[cfg(feature = "BiRegularCollapseVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCollapseVertical")]
/// *This icon requires the feature* `BiRegularCollapseVertical` *to be enabled*.
#[component]
pub fn CollapseVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 7.59 7.05 2.64 5.64 4.05 12 10.41l6.36-6.36-1.41-1.41L12 7.59zM5.64 19.95l1.41 1.41L12 16.41l4.95 4.95 1.41-1.41L12 13.59l-6.36 6.36z" /></svg>
   }
}