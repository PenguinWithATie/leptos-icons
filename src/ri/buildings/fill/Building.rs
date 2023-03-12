#[cfg(feature = "RiBuildingsFillBuilding")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBuildingsFillBuilding")]
/// *This icon requires the feature* `RiBuildingsFillBuilding` *to be enabled*.
#[component]
pub fn Building(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 19h2v2H1v-2h2V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v15h2V9h3a1 1 0 0 1 1 1v9zM7 11v2h4v-2H7zm0-4v2h4V7H7z" /></g></svg>
   }
}