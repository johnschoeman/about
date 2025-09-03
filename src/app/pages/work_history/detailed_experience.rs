use leptos::prelude::*;

#[component]
pub fn DetailedExperience() -> impl IntoView {
    view! {
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
    }
}
