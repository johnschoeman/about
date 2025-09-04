use leptos::prelude::*;

#[component]
pub fn WorkSummary() -> impl IntoView {
    view! {
        <div class="mt-24">
            <h2 class="text-4xl font-black text-gray-900">"in short"</h2>
            <div>"professional focus at a glance"</div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "2017 - 2025"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Principal Engineer & Technical Leader"</h3>
                    <p class="mt-2">
                        "I work as a principal engineer and technical leader specializing in "
                        "frontend architecture, team leadership, and engineering culture transformation. "
                        "I focus on TypeScript, React, functional programming, and modern deployment practices."
                    </p>

                    <p class="mt-2">
                        "I've successfully scaled engineering teams, established continuous delivery practices, "
                        "and led complete product rewrites while maintaining exceptional quality standards "
                        "and mentoring engineers from mid to principal level."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "2014 - 2017"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Structural Engineer"</h3>
                    <p class="mt-2">
                        "I worked as a structural engineer in the states of California and "
                        "Massachusetts. I helped create the design of and construction "
                        "documents for structural systems of commerical and public building "
                        "superstructures and foundations."
                    </p>
                    <p class="mt-2">
                        "An example project I worked on is "
                        <a
                            class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                            href="https://www.rwlasvegas.com/"
                        >
                            "Resorts World, Las Vegas"
                        </a>
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "2008 - 2013"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Engineering and Mathematics Student"</h3>
                    <p class="mt-2">
                        "I went to school at Cal Poly in San Luis Obispo, where I studied "
                        "Architectural Engineering and Mathematics. I graduated with the cum "
                        "laude distinction."
                    </p>
                </div>
            </div>
        </div>
    }
}
