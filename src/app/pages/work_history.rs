use leptos::prelude::*;

#[component]
pub fn WorkHistory() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"work history"</h1>
            <div>"from structural engineer to product developer"</div>

            <div class="text-gray-900 leading-normal break-words">
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

                <div class="mt-24">
                    <h2 class="text-4xl font-black text-gray-900">"in some detail"</h2>
                    <div>"from apprentice to team lead"</div>

                    <div class="mt-12">
                        <h3 class="text-2xl font-semibold text-gray-900">"Evolv Technologies"</h3>
                        <div>"6 months"</div>

                        <p class="mt-8 text-gray-900">
                            "I work as a Technical Team Lead and product developer at "
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                              href="https://www.evolvtechnology.com/">"Evolv"</a>
                            " in the UI/UX division. "
                            "Professionally, I spend most of my time developing digital products and "
                            "helping teams develop better software practices."
                        </p>

                        <p class="mt-2">
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600" href="https://www.evolvtechnology.com/">"Evolv"</a>
                            " is a technology company in the safety and security space. We are developing "
                            "the next generation weapon detection systems which incorporate novel "
                            "sensor technologies, machine learning, and integrated analytics."
                        </p>
                    </div>

                    <div class="mt-12">
                        <h3 class="text-2xl font-semibold text-gray-900">"thoughtbot"</h3>
                        <div>"3 years, 6 months"</div>

                        <p class="mt-2">
                            <a class="text-blue-600 underline hover:text-blue-800 visited:text-indigo-600" href="https://thoughtbot.com/">"thoughtbot"</a>
                            " is a 100+ person consultancy focusing on the design and development of digital products."
                        </p>

                        <p class="mt-2">
                            "Clients hire thoughtbot developers and designers to bring new products "
                            "to market and help make existing products more successful."
                        </p>

                        <p class="mt-2">
                            "Our developers are all product-oriented, and our designers are all programmers."
                        </p>

                        <p class="mt-2">
                            "We work with a various technologies but our most common tech stacks are "
                            "in the JavaScript and Ruby on Rails ecosystems."
                        </p>

                        <p class="mt-2">
                            "For all the roles below, I worked as a full-stack individual technical "
                            "contributor and consultant, providing technical expertise for multiple "
                            "clients ranging from start-ups to established growth-stage companies. I "
                            "primarily contributed to clients' business success by delivering quality "
                            "application code daily and by improving engineering team processes."
                        </p>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Technical Team Lead"</h4>
                            <div class="text-sm text-gray-700">"Dec 2021 - Now (3 mos) · Boston, MA"</div>
                            <p class="mt-2">
                                "Technical Team Lead for the UI/UX division, which focuses on Evolv's "
                                "digital product suite that allows users to interact with their fleet of "
                                "scanners."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>
                                    "Establish and implement Software Delivery process for web "
                                    "applications at Evolv."
                                </li>
                                <li class="mt-1">
                                    "Design and develop new product features."
                                </li>
                                <li class="mt-1">
                                    "Work closely with the managing director to manage team and establish "
                                    "hiring practices for front end developers."
                                </li>
                                <li class="mt-1">"Established technical strategy for the team"</li>
                                <li class="mt-1">
                                    "Managed other developers, including weekly 1-on-1s, reviewing for "
                                    "promotions, and providing general support"
                                </li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Senior Frontend Developer"</h4>
                            <div class="text-sm text-gray-700">"Sept 2021 - Dec 2021 (2 mos) · Boston, MA"</div>
                            <p class="mt-2">
                                "Senior Frontend Developer in Evolv's UI/UX division, which focuses on Evolv's "
                                "digital product suite that allows users to interact with their fleet of "
                                "scanners."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>
                                    "Establish and implement Software Delivery process for web "
                                    "applications at Evolv."
                                </li>
                                <li class="mt-1">
                                    "Design and develop new product features."
                                </li>
                                <li class="mt-1">
                                    "Helped transition legacy codebase to modern development standards."
                                </li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Ignite Development Team Lead"</h4>
                            <div class="text-sm text-gray-700">"Jan 2021 - Sept 2021 (8 mos) · Remote, (GMT-4)"</div>
                            <p class="mt-2">
                                "Development Team Lead for the ignite team specializing in early-stage "
                                "proof of concepts and first product iterations. My role breaks down as "
                                "70% individual contributor and 30% managerial."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>
                                    "Work on client projects as a technical contributor and project "
                                    "manager"
                                </li>
                                <li class="mt-1">
                                    "Work as an advisor to other projects where I facilitate client "
                                    "meetings and help to keep the project on schedule and budget"
                                </li>
                                <li class="mt-1">
                                    "Work closely with the managing director to scope new projects and "
                                    "create project proposals for new clients"
                                </li>
                                <li class="mt-1">"Established technical strategy for the business unit"</li>
                                <li class="mt-1">
                                    "Managed other developers, including weekly 1-on-1s, reviewing for "
                                    "promotions, and providing general support"
                                </li>
                                <li class="mt-1">
                                    "Established hiring strategy and technical interview process for new "
                                    "developers on the Ignite team"
                                </li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Development Team Lead"</h4>
                            <div class="text-sm text-gray-700">"Mar 2020 - Jan 2021 (8 mos) · Boston, MA"</div>
                            <p class="mt-2">
                                "Development Team Lead for the Boston studio. The role broke down as "
                                "90% individual contributor and 10% managerial."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>
                                    "Work on client projects as a technical contributor and project "
                                    "manager"
                                </li>
                                <li class="mt-1">
                                    "Worked closely with the Managing Director to staff projects "
                                    "appropriately, maintain team member fulfillment, and define the "
                                    "direction for the studio."
                                </li>
                                <li class="mt-1">
                                    "Managed other developers, providing technical and interpersonal "
                                    "support and guidance, having weekly 1-on-1s, and reviewing for "
                                    "promotions"
                                </li>
                                <li class="mt-1">"Responsible for interviewing new developers for the team"</li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Senior Developer"</h4>
                            <div class="text-sm text-gray-700">"Sep 2019 - Mar 2020 (6 mos) · Boston, MA"</div>
                            <p class="mt-4">
                                "Senior Developer in the Boston studio. My role was primarily ensuring "
                                "the technical success of client projects."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>"Expected to lead and work solo on client projects"</li>
                                <li class="mt-1">
                                    "Worked with product owners to establish product roadmaps, especially "
                                    "with input from a technical perspective"
                                </li>
                                <li class="mt-1">
                                    "Brokedown high-level business goals into shippable product "
                                    "iterations"
                                </li>
                                <li class="mt-1">
                                    "Worked with other product developers and designers to build and ship "
                                    "new product features"
                                </li>
                                <li class="mt-1">
                                    "Mentored junior engineers on client teams on best practices, "
                                    "including design patterns and test driven development"
                                </li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Developer"</h4>
                            <div class="text-sm text-gray-700">"Jun 2018 - Sep 2019 (1 yr 4 mos) · Boston, MA"</div>
                            <p class="mt-4">
                                "Developer at thoughtbot. I worked closely with clients on various "
                                "projects as a technical contributor helping to make the project more "
                                "successful."
                            </p>
                            <ul class="mt-4 text-base pl-8 list-disc">
                                <li>
                                    "Brokedown high-level business goals into shippable product "
                                    "iterations"
                                </li>
                                <li class="mt-1">
                                    "Worked closely with designers and product managers to implement and "
                                    "ship successful product solutions for users and clients."
                                </li>
                                <li class="mt-1">
                                    "Implemented best practices for software delivery, including "
                                    "continuous integration and continuous deployment"
                                </li>
                                <li class="mt-1">
                                    "Helped clients identify potential technical solutions for new "
                                    "products and improve existing applications by addressing technical "
                                    "debt"
                                </li>
                                <li class="mt-1">
                                    "Improved application performance by identifying and addressing non "
                                    "performant code and database queries"
                                </li>
                                <li class="mt-1">
                                    "Contributed to the technology community by giving technical "
                                    "presentations, writing blog posts, and organizing for the Boston "
                                    "Ruby, Elixir, and Scala Meetup groups."
                                </li>
                            </ul>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Apprentice"</h4>
                            <div class="text-sm text-gray-700">"Mar 2018 - Jun 2018 (3 mos) · Boston, MA"</div>
                            <p class="mt-2">
                                "Apprentice at thoughtbot. Apprentices work closely with thoughtbot "
                                "developer mentors on client projects to develop the required skill set "
                                "to be billable. I focused on learning the advanced patterns and "
                                "practices used by thoughtbot developers to ensure quality and reliable "
                                "software and software delivery methods."
                            </p>

                            <h5 class="mt-4 mb-2 text-sm font-semibold text-gray-900">"Skills focused on:"</h5>
                            <ul class="text-base pl-8 list-disc">
                                <li>"Test-Driven Development"</li>
                                <li class="mt-1">"Production software patterns"</li>
                                <li class="mt-1">"TypeScript"</li>
                            </ul>
                        </div>
                    </div>

                    <div class="mt-12">
                        <h3 class="text-2xl font-semibold text-gray-900">"DeSimone Consulting Engineers"</h3>
                        <div>"2 years, 11 months"</div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Project Engineer"</h4>
                            <div class="text-sm text-gray-700">"Aug 2016 - July 2017 (1 yr) · Boston, MA"</div>

                            <p class="mt-4">
                                "Created and maintained software programs used by other engineers to "
                                "automate the analysis of wind design loads, design of long-span "
                                "structural members, design high-strength connections compliant with "
                                "the California Building Code."
                            </p>

                            <p class="mt-4">
                                "Responsible for design solutions and analysis used to produce "
                                "construction documents for building structural systems. Projects are "
                                "primarily in the commercial sector."
                            </p>
                        </div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Project Engineer"</h4>
                            <div class="text-sm text-gray-700">"Sep 2014 - Aug 2016 (2 yrs) · Las Vegas, NV"</div>
                            <p class="mt-4">
                                "Responsible for design solutions and analysis used to produce "
                                "construction documents for building structural systems. Projects are "
                                "primarily in the hospitality sector and included The Surf Club in "
                                "Miami by architect Richard Meiers and the Resorts World Las Vegas "
                                "project by Gensler."
                            </p>
                        </div>
                    </div>

                    <div class="mt-12">
                        <h3 class="text-2xl font-semibold text-gray-900">"Lionakis"</h3>
                        <div>"9 months"</div>

                        <div class="mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8">
                            <h4 class="leading-tight text-base font-semibold text-gray-900">"Designer/Project Engineer"</h4>
                            <div class="text-sm text-gray-700">"Jan 2014 - Aug 2014 (9 mos) · Sacramento, CA"</div>
                            <p class="mt-4">
                                "Responsible for design solutions and analysis used to produce "
                                "construction documents for building structural systems. Projects are "
                                "primarily educational and healthcare facilities. Performed analysis of "
                                "existing and new structures for code and structural compliance."
                            </p>
                        </div>
                    </div>
                </div>

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
            </div>
        </div>
    }
}
