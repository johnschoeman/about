use leptos::prelude::*;

#[component]
pub fn FullTimeline() -> impl IntoView {
    view! {
        <div class="mt-24">
            <h2 class="text-4xl font-black text-gray-900">"in more detail"</h2>
            <div>"individual contributions and full timeline"</div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "May 2021 - Now"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Fraiser"</h3>
                    <div class="mb-2 text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "Svelte • InertiaJS • TypeScript • Ruby on Rails • Tailwindcss • Postgres • Heroku • S3"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Mar 2021 - May 2021"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Tradewinds"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "StimulusJS • TypeScript • Ruby on Rails • Tailwindcss • LeafletJS • Postgres • Heroku"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Jan 2021 - Mar 2021"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Epic Games: Houseparty"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "React • TypeScript • StyledComponents"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Jan 2021"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot Ignite - Development Team Lead"</h3>
                    <div class="text-sm text-gray-700">"Promotion"</div>
                    <p class="mt-2">
                        "In January 2021, thoughtbot restructures from city-based studios to "
                        "service offering based studios. I joined the Ignite team, specializing "
                        "in the earliest stage products, continuing as a Development Team Lead. "
                        "I am the most senior developer on the team. My duties expanded to "
                        "include establishing the technical strategy for the team and the "
                        "hiring strategy for new developers."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "May 2020 - Jan 2021"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"PathCheck"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "React Native • TypeScript • Native APIs • Swift • Fastlane"
                    </span>
                    <div class="mt-2">
                        <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600 mr-4" href="https://thoughtbot.com/case-studies/pathcheck">"Case Study"</a>
                        <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600" href="https://github.com/Path-Check/gaen-mobile">"github"</a>
                    </div>
                    <p class="mt-2">
                        "I was a core contributor and technical project manager for PathCheck's "
                        "Covid-19 contact tracing consumer app project. Over a few months, we "
                        "designed, developed, and shipped the application to 7 states and "
                        "countries, which had millions of downloads and helped fight Covid-19."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Mar 2020"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot Development Team Lead"</h3>
                    <div class="text-sm text-gray-700">"Promotion"</div>
                    <p class="mt-2">
                        "In Q1 of 2020, thoughtbot introduces the Development Team Lead "
                        "position. I am included among the initial group of developers to "
                        "receive the promotion. The role continues to have four days a week of "
                        "working on client projects as an individual contributor and adds the "
                        "new responsibility of directly managing 2 to 3 other developers."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Feb 2020 - May 2020"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"HomeMinder Mobile MVP"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "PWA • Elm • Capacitor • Tailwindcss • NodeJS • Postgres • Netlify"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Feb 2020 - May 2020"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot Senior Developer"</h3>
                    <div class="text-sm text-gray-700">"Promotion"</div>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Aug 2019 - Jan 2020"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"AiRXOS Mobile Applications"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "React Native • TypeScript • Redux"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Apr 2019 - Aug 2019"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Careport"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "Ruby on Rails • JQuery • SCSS"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Nov 2018 - Mar 2019"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"ComplianceMate"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "Ruby on Rails • AWS • Postgres"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Oct 2018 - Nov 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Splitfit"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "React Native"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Jun 2018 - Sep 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Careport"</h3>
                    <div class="text-sm text-gray-700">"Client Project"</div>
                    <span class="font-semibold text-gray-700">
                        "Ruby on Rails • Elm • Postgres"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Jun 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot Developer"</h3>
                    <div class="text-sm text-gray-700">"Promotion"</div>
                    <p class="mt-2">
                        "After completing my apprenticeship, I was offered a developer role at "
                        "thoughtbot in their Boston studio."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "May 2018 - Jun 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Mass Challenge"</h3>
                    <div class="text-sm text-gray-700">"Apprenticeship - Month 3"</div>
                    <span class="font-semibold text-gray-700">
                        "ReactJS • TypeScript"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Apr 2018 - May 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"JoyDrive"</h3>
                    <div class="text-sm text-gray-700">"Apprenticeship - Month 2"</div>
                    <span class="font-semibold text-gray-700">
                        "Phoenix • Elixir"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Mar 2018 - Apr 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Careport"</h3>
                    <div class="text-sm text-gray-700">"Apprenticeship - Month 1"</div>
                    <span class="font-semibold text-gray-700">
                        "Ruby on Rails • Elm • Heroku"
                    </span>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Feb 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot Apprenticeship"</h3>
                    <p class="mt-2">
                        "I applied to thoughtbot as a developer and received an offer for their "
                        "apprenticeship program. The thoughtbot apprenticeship is a three-month "
                        "paid program offered to newer developers to help build out their "
                        "technical and consulting skillsets. While an apprentice, I learned "
                        "best practices from senior thoughtbot developers, including "
                        "test-driven development, vim, team management, and software delivery "
                        "methods."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Sep 2017 - Feb 2018"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Independent Contracting"</h3>
                    <span class="font-semibold text-gray-700">
                        "React Native"
                    </span>
                    <p class="mt-2">
                        "After graduating App Academy, I started my job search and worked as a "
                        "part-time independent contractor for a start-up helping build out "
                        "their React Native application."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Fall 2017"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"App Academy"</h3>
                    <div>"San Francisco, CA"</div>

                    <p class="mt-2">
                        "A highly selective and rigorous software engineering program focusing "
                        "on Ruby on Rails and ReactJS web development."
                    </p>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Aug 2016 - July 2017"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Desimone Consulting Engineers"</h3>
                    <div>"Boston, MA"</div>
                    <ul class="mt-4 text-base pl-8 list-disc">
                        <li>
                            "Designed structural gravity and lateral systems to meet "
                            "architectural and code requirements"
                        </li>

                        <li class="mt-1">
                            "Responded to and made recommendations for RFI's and urgent "
                            "change-orders"
                        </li>

                        <li class="mt-1">
                            "Designed connection and member details for concrete and wood "
                            "construction"
                        </li>

                        <li class="mt-1">
                            "Reviewed and made recommendations on documents by other "
                            "professionals including post-tensioned concrete reinforcement, mild "
                            "reinforcement concrete, and steel connections"
                        </li>
                    </ul>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Sep 2014 - Aug 2016"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Desimone Consulting Engineers"</h3>
                    <div>"Las Vegas, NV"</div>
                    <ul class="mt-4 text-base pl-8 list-disc">
                        <li>
                            "Designed structural gravity and lateral systems to meet "
                            "architectural and code requirements"
                        </li>

                        <li class="mt-1">
                            "Responded to and made recommendations for RFI's and urgent "
                            "change-orders"
                        </li>

                        <li class="mt-1">
                            "Designed connection and member details for concrete and wood "
                            "construction"
                        </li>

                        <li class="mt-1">
                            "Reviewed and made recommendations on documents by other "
                            "professionals, including post-tensioned concrete reinforcement, mild "
                            "reinforcement concrete, and steel connections"
                        </li>
                    </ul>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "Jul 2014 - Aug 2014"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"Lionakis"</h3>
                    <div>"Sacramento, CA"</div>
                    <ul class="mt-4 text-base pl-8 list-disc">
                        <li>
                            "Selected and designed structural lateral and gravity systems to meet "
                            "architectural and code requirements"
                        </li>
                        <li class="mt-1">
                            "Reviewed and made recommendations on documents by other "
                            "professionals"
                        </li>
                        <li class="mt-1">"Reviewed and approved shop drawings for steel elements"</li>
                        <li class="mt-1">
                            "Utilized drafting skills for wood, steel, and concrete construction "
                            "document development"
                        </li>
                        <li class="mt-1">"Responded to review agency comments."</li>
                        <li class="mt-1">"Managed project deadlines"</li>
                    </ul>
                </div>
            </div>

            <div class="flex mt-8 flex-col sm:flex-row">
                <div class="flex border-gray-300 sm:w-44">
                    <div class="w-full text-sm text-gray-700 sm:text-right leading-10">
                        "2008 - 2013"
                    </div>
                </div>
                <div class="sm:pl-8 w-full">
                    <h3 class="text-2xl font-semibold text-gray-900">"School"</h3>
                    <p class="mt-2">"California Polytech University"</p>
                    <p class="mt-2">"San Luis Obispo, CA"</p>
                    <p class="mt-2">"Architectural Engineering Major"</p>
                    <p class="mt-2">"Mathematics Minor"</p>
                    <p class="mt-2">"Cum Laude"</p>
                </div>
            </div>
        </div>
    }
}
