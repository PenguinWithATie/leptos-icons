#[cfg(feature = "BiRegularCuboid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCuboid")]
/// *This icon requires the feature* `BiRegularCuboid` *to be enabled*.
#[component]
pub fn Cuboid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.707 2.293A.996.996 0 0 0 16 2H8c-.414 0-.785.255-.934.641l-5 13a.999.999 0 0 0 .227 1.066l5 5A.996.996 0 0 0 8 22h8c.414 0 .785-.255.934-.641l5-13a.999.999 0 0 0-.227-1.066l-5-5zM18.585 7h-5.171l-3-3h5.172l2.999 3zM8.381 4.795l3.438 3.438-4.462 10.71-3.19-3.191L8.381 4.795zM15.313 20h-6.23l4.583-11h5.878l-4.231 11z" /></svg>
   }
}