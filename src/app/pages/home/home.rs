use leptos::prelude::*;

use super::ortho_board_doodle::OrthoBoardDoodle;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-4xl sm:text-8xl font-black text-gray-900">"hello!"</h1>

            <p class="mt-4 sm:mt-8 text-gray-900">
                "I'm a software developer with expertise in full stack web development."
            </p>

            <p class="mt-6 text-gray-900">
                "Professionally, I spend most of my time building digital products and helping teams develop better software practices. "
                "I have also contributed as a designer, project manager, technical team lead, and people manager (not all at the same time)."
            </p>

            <p class="mt-6 text-gray-900">
                "I'm currently taking a sabbatical to dive deeper into creative projects and emerging technologies, "
                "particularly Rust, Nix, and higher-level mathematics."
            </p>

            <p class="mt-6 text-gray-900">
                "Open to collaborating on interesting projects and discussing opportunities with teams "
                "who value craftsmanship and thoughtful engineering."
            </p>

            <div class="mt-24">
                <OrthoBoardDoodle />
            </div>
        </div>
    }
}
