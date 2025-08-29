use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"hello, i'm john."</h1>

            <p class="mt-8 text-gray-900">
                "I work as a Software Developer, with a focus on web development. "
                "Professionally, I spend most of my time developing digital products and "
                "helping teams develop better software practices."
            </p>

            <div class="mt-64">
                // OrthoBoardDoodle component would go here
                // For now, placeholder content
                <div class="w-full h-32 bg-gray-200 border border-gray-300 rounded">
                    <div class="flex items-center justify-center h-full text-gray-500">
                        "Interactive Doodle Placeholder"
                    </div>
                </div>
            </div>
        </div>
    }
}
