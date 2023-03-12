#[cfg(feature = "RiDeviceFillBattery2Charge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillBattery2Charge")]
/// *This icon requires the feature* `RiDeviceFillBattery2Charge` *to be enabled*.
#[component]
pub fn Battery2Charge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 4V3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v1h3a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h3zm4 8V7l-5 7h3v5l5-7h-3z" /></g></svg>
   }
}