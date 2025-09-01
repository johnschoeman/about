use leptos::prelude::*;

use super::ortho_board_doodle::OrthoBoardDoodle;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"hello, i'm john."</h1>

            <p class="mt-8 text-xl text-gray-900">
                "Senior Software Developer specializing in web technologies and systems programming."
            </p>

            <p class="mt-6 text-gray-900">
                "I build robust digital products and help teams ship better software. Currently exploring "
                "the intersection of web development and systems programming, with a focus on Rust and Nix. "
                "Taking a sabbatical to dive deeper into creative projects and emerging technologies."
            </p>

            <p class="mt-6 text-gray-900">
                "Open to collaborating on interesting projects and discussing opportunities with teams "
                "who value craftsmanship, innovation, and thoughtful engineering."
            </p>

            <div class="mt-12">
                <OrthoBoardDoodle />
            </div>
        </div>
    }
}
