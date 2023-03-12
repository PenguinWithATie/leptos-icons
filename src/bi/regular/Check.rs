#[cfg(feature = "BiRegularCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCheck")]
/// *This icon requires the feature* `BiRegularCheck` *to be enabled*.
#[component]
pub fn Check(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m10 15.586-3.293-3.293-1.414 1.414L10 18.414l9.707-9.707-1.414-1.414z" /></svg>
   }
}