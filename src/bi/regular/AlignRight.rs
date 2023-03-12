#[cfg(feature = "BiRegularAlignRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlignRight")]
/// *This icon requires the feature* `BiRegularAlignRight` *to be enabled*.
#[component]
pub fn AlignRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 19h16v2H4zm5-4h11v2H9zm-5-4h16v2H4zm0-8h16v2H4zm5 4h11v2H9z" /></svg>
   }
}