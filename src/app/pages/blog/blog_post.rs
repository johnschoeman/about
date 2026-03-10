use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use super::blog_posts::find_post;

#[component]
pub fn BlogPostView() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug");

    move || {
        let Some(slug) = slug() else {
            return view! { <p>"Post not found."</p> }.into_any();
        };

        match find_post(&slug) {
            Some(post) => view! {
                <article>
                    <h1 class="text-5xl font-black text-gray-900">{post.title}</h1>
                    <p class="mt-2 text-sm text-gray-500">{post.date}</p>
                    <div class="prose mt-8" inner_html=post.render_html()></div>
                </article>
            }.into_any(),
            None => view! { <p>"Post not found."</p> }.into_any(),
        }
    }
}
