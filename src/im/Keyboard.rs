#[cfg(feature = "ImKeyboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImKeyboard")]
/// *This icon requires the feature* `ImKeyboard` *to be enabled*.
#[component]
pub fn Keyboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="18" height="16" viewBox="0 0 18 16"><path fill="#000000" d="M17 2h-16c-0.55 0-1 0.45-1 1v10c0 0.55 0.45 1 1 1h16c0.55 0 1-0.45 1-1v-10c0-0.55-0.45-1-1-1zM10 4h2v2h-2v-2zM13 7v2h-2v-2h2zM7 4h2v2h-2v-2zM10 7v2h-2v-2h2zM4 4h2v2h-2v-2zM7 7v2h-2v-2h2zM2 4h1v2h-1v-2zM2 7h2v2h-2v-2zM3 12h-1v-2h1v2zM12 12h-8v-2h8v2zM16 12h-3v-2h3v2zM16 9h-2v-2h2v2zM16 6h-3v-2h3v2z" /></svg>
   }
}