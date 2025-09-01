use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-gray-300">
            <div class="max-w-4xl px-4 sm:px-8 py-8 mx-auto flex flex-wrap gap-4 sm:space-x-4 sm:gap-0">
                <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                   href="https://www.linkedin.com/in/john-schoeman-36894658/">
                    "linkedin"
                </a>
                <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                   href="https://github.com/johnschoeman">
                    "github"
                </a>
                <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                   href="https://twitter.com/j_schoeman">
                    "twitter"
                </a>
                <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                   href="https://www.instagram.com/johns1729/">
                    "instagram"
                </a>
            </div>
        </footer>
    }
}
