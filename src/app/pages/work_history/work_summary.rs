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
                        "2017 - Now"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Product Developer"</h3>
                    <p class="mt-2">
                        "I currently work as a full-stack developer and product consultant for "
                        "digital products on web and mobile platforms. I mostly use "
                        "technologies in the JavaScript ecosystem."
                    </p>

                    <p class="mt-2">
                        "While primarily working as an individual technical contributor, I have "
                        "also contributed as project manager, ux designer, and people manager."
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
