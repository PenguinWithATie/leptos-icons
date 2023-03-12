#[cfg(feature = "TbBrandLoom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandLoom")]
/// *This icon requires the feature* `TbBrandLoom` *to be enabled*.
#[component]
pub fn BrandLoom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-loom" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17.464 6.518a6 6 0 1 0 -3.023 7.965" /><path d="M17.482 17.464a6 6 0 1 0 -7.965 -3.023" /><path d="M6.54 17.482a6 6 0 1 0 3.024 -7.965" /><path d="M6.518 6.54a6 6 0 1 0 7.965 3.024" /></svg>
   }
}