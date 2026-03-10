use leptos::prelude::*;

use super::blog_posts::all_posts;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = all_posts();

    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"blog"</h1>

            <ul class="mt-8">
                {posts
                    .into_iter()
                    .map(|post| {
                        let href = format!("/blog/{}", post.slug);
                        view! {
                            <li class="flex flex-row items-baseline gap-4 mt-4">
                                <span class="text-sm text-gray-500">{post.date}</span>
                                <a
                                    class="text-xl text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                                    href=href
                                >
                                    {post.title}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
