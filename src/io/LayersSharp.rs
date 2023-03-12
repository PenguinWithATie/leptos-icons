#[cfg(feature = "IoLayersSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLayersSharp")]
/// *This icon requires the feature* `IoLayersSharp` *to be enabled*.
#[component]
pub fn LayersSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="480 150 256 48 32 150 256 254 480 150" /><polygon points="255.71 392.95 110.9 326.75 32 362 256 464 480 362 401.31 326.7 255.71 392.95" /><path d="M480,256l-75.53-33.53L256.1,290.6,107.33,222.43,32,256,256,358,480,256S480,256,480,256Z" /></svg>
   }
}