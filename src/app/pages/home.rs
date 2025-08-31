use crate::app::components::ortho_board_doodle::OrthoBoardDoodle;
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
                <OrthoBoardDoodle />
            </div>
        </div>
    }
}
