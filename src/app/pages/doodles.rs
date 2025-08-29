use leptos::prelude::*;

#[component]
pub fn Doodles() -> impl IntoView {
    let doodles = vec![
        ("doodles.camp 🏕️", "https://www.doodles.camp/"),
        ("doodles.kitchen 🍳", "https://www.doodles.kitchen/"),
    ];

    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"doodles"</h1>
            <p class="mt-2 text-sm text-gray-700">"a collection of programming 'doodles'"</p>

            <ul class="mt-8">
                {doodles
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (label, url))| {
                        let idx_str = format!("{:02}", idx);
                        view! {
                            <li class="flex flex-row items-center mt-8">
                                <p class="w-12 h-full text-2xl font-bold text-gray-400 align-middle">
                                    {idx_str}
                                </p>
                                <a
                                    class="text-2xl align-middle text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                                    href={url}
                                >
                                    {label}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
