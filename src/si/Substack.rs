#[cfg(feature = "SiSubstack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSubstack")]
/// *This icon requires the feature* `SiSubstack` *to be enabled*.
#[component]
pub fn Substack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M22.539 8.242H1.46V5.406h21.08v2.836zM1.46 10.812V24L12 18.11 22.54 24V10.812H1.46zM22.54 0H1.46v2.836h21.08V0z" /></svg>
   }
}