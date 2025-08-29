use leptos::prelude::*;

#[component]
pub fn WorkHistory() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"work history"</h1>
            <div>"from structural engineer to product developer"</div>

            <div class="markdown">
                <div class="mt-24">
                    <h2>"in short"</h2>
                    <div>"professional focus at a glance"</div>

                    <div class="flex mt-8 flex-col sm:flex-row">
                        <div class="flex border-gray-300 sm:w-44">
                            <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                                "2017 - Now"
                            </div>
                        </div>
                        <div class="sm:pl-8 w-full">
                            <h3>"Product Developer"</h3>
                            <p>
                                "I currently work as a full-stack developer and product consultant for "
                                "digital products on web and mobile platforms. I mostly use "
                                "technologies in the JavaScript ecosystem."
                            </p>

                            <p>
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
                            <h3>"Structural Engineer"</h3>
                            <p>
                                "I worked as a structural engineer in the states of California and "
                                "Massachusetts. I helped create the design of and construction "
                                "documents for structural systems of commerical and public building "
                                "superstructures and foundations."
                            </p>
                            <p>
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
                            <h3>"Engineering and Mathematics Student"</h3>
                            <p>
                                "I went to school at Cal Poly in San Luis Obispo, where I studied "
                                "Architectural Engineering and Mathematics. I graduated with the cum "
                                "laude distinction."
                            </p>
                        </div>
                    </div>
                </div>

                <div class="mt-24">
                    <h2>"in some detail"</h2>
                    <div>"from apprentice to team lead"</div>

                    <div class="mt-12">
                        <h3>"Evolv Technologies"</h3>
                        <div>"6 months"</div>

                        <p class="mt-8 text-gray-900">
                            "I work as a Technical Team Lead and product developer at "
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                              href="https://www.evolvtechnology.com/">"Evolv"</a>
                            " in the UI/UX division. "
                            "Professionally, I spend most of my time developing digital products and "
                            "helping teams develop better software practices."
                        </p>

                        <p>
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600" href="https://www.evolvtechnology.com/">"Evolv"</a>
                            " is a technology company in the safety and security space. We are developing "
                            "the next generation weapon detection systems which incorporate novel "
                            "sensor technologies, machine learning, and integrated analytics."
                        </p>
                    </div>

                    <div class="mt-12">
                        <h3>"thoughtbot"</h3>
                        <div>"3 years, 6 months"</div>

                        <p>
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600" href="https://thoughtbot.com/">"thoughtbot"</a>
                            " is a 100+ person consultancy focusing on the design and development of digital products."
                        </p>

                        <p>
                            "Clients hire thoughtbot developers and designers to bring new products "
                            "to market and help make existing products more successful."
                        </p>

                        <p>
                            "Our developers are all product-oriented, and our designers are all programmers."
                        </p>

                        <p>
                            "We work with a various technologies but our most common tech stacks are "
                            "in the JavaScript and Ruby on Rails ecosystems."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
