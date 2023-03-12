#[cfg(feature = "BiRegularExitFullscreen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExitFullscreen")]
/// *This icon requires the feature* `BiRegularExitFullscreen` *to be enabled*.
#[component]
pub fn ExitFullscreen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 4H8v4H4v2h6zM8 20h2v-6H4v2h4zm12-6h-6v6h2v-4h4zm0-6h-4V4h-2v6h6z" /></svg>
   }
}