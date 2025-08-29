use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"about"</h1>

            <div class="mt-8">
                <h2 class="mb-2 text-4xl font-black text-gray-900">"development work"</h2>

                <a
                    class="mt-4 text-lg text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                    href="https://apps.apple.com/us/app/building-habits/id1534446576"
                >
                    "Each"
                </a>

                <p class="mt-2 text-gray-900">
                    "A super simple app designed to help build better habits, built with React Native."
                </p>
            </div>

            <div class="mt-8">
                <h2 class="text-4xl font-black text-gray-900">"writing and talks"</h2>

                <a
                    class="mt-4 text-lg text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                    href="https://podcasts.apple.com/us/podcast/trying-to-build-good-things/id1533617135"
                >
                    "trying to build good things"
                </a>

                <p class="mt-2 text-gray-900">
                    "A podcast where my good friend and I discuss designing and developing "
                    "quality products. We've structured it around the journey of an idea to a "
                    "live product. Listen in if you are interested in hearing our thoughts about "
                    "how to build good things."
                </p>

                <a
                    class="mt-4 text-lg text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                    href="https://thoughtbot.com/blog/structure-for-styling-in-react-native"
                >
                    "React Native Styling: Structure for Style Organization"
                </a>
                <p class="mt-0 text-sm text-gray-700">"blog post"</p>

                <a
                    class="mt-4 text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                    href="https://www.youtube.com/watch?v=toSedSFnzOE"
                >
                    "Sprinkles of Functional Programming"
                </a>
                <p class="text-sm text-gray-700">"rails conf 2019"</p>
            </div>
        </div>
    }
}
