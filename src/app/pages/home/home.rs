use leptos::prelude::*;

use super::ortho_board_doodle::OrthoBoardDoodle;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-4xl sm:text-6xl font-black text-gray-900">"hello,"</h1>

            <p class="mt-4 sm:mt-8 text-gray-900">
                "I'm John. I'm a software developer and this is my personal website."
            </p>

            <p class="mt-6 text-gray-900">
                "Professionally, I spend most of my time building digital products and helping teams develop better software practices. Currently exploring "
                "the intersection of web development and systems programming, with a focus on Rust and Nix. "
                "Taking a sabbatical to dive deeper into creative projects and emerging technologies."
            </p>

            <p class="mt-6 text-gray-900">
                "Open to collaborating on interesting projects and discussing opportunities with teams "
                "who value craftsmanship and thoughtful engineering."
            </p>

            <div class="mt-12">
                <OrthoBoardDoodle />
            </div>
        </div>
    }
}
