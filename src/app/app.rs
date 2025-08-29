use leptos::prelude::*;

use crate::app::header::Header;
use crate::app::footer::Footer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-full max-w-4xl px-8 pb-24 mx-auto mb-auto">
            <Header />
            <p>Hello World</p>
             <Footer />
         </main>
    }
}

