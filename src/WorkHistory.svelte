<script lang="ts">
  import DateRange from "./DateRange.svelte"
  import RoleDetails from "./RoleDetails.svelte"
  import TechnologyList from "./TechnologyList.svelte"

  const timelineListItemContainerStyle = "flex mt-8 flex-col sm:flex-row"

  const timelineLeftContainerStyle = "flex border-gray-300 sm:w-44"

  const timelineRightContainerStyle = "sm:pl-8 w-full"

  const someDetailJobContainerStyle =
    "mt-8 pl-4 border-gray-100 border-l-2 sm:pl-8"
</script>

<h1 class="h1">work history</h1>
<div>from structural engineer to product developer</div>

<div class="markdown">
  <div class="mt-24">
    <h2>in short</h2>
    <div>professional focus at a glance</div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"2017"} to={"Now"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Product Developer</h3>
        <p>
          I currently work as a full-stack developer and product consultant for
          digital products on web and mobile platforms. I mostly use
          technologies in the JavaScript ecosystem.
        </p>

        <p>
          While primarily working as an individual technical contributor, I also
          contribute as project manager, ux designer, and people manager.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"2014"} to={"2017"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Structural Engineer</h3>
        <p>
          I worked as a structural engineer in the states of California and
          Massachusetts. I helped create the design of and construction
          documents for structural systems of commerical and public building
          superstructures and foundations.
        </p>
        <p>
          An example project I worked on is <a
            class="lnk"
            href="https://www.rwlasvegas.com/">Resorts World, Las Vegas</a
          >
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"2008"} to={"2013"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Engineering and Mathematics Student</h3>
        <p>
          I went to school at Cal Poly in San Luis Obispo, where I studied
          Architectural Engineering and Mathematics. I graduated with the cum
          laude distinction.
        </p>
      </div>
    </div>
  </div>

  <div class="mt-24">
    <h2>in some detail</h2>
    <div>from apprentice to team lead</div>

    <div class="mt-12">
      <h3>thoughtbot</h3>
      <div>3 years, 6 months</div>

      <p>
        <a class="lnk" href="https://thoughtbot.com/">thoughtbot</a>
        is a 100+ person consultancy focusing on the design and development of digital
        products.
      </p>

      <p>
        Clients hire thoughtbot developers and designers to bring new products
        to market and help make existing products more successful.
      </p>

      <p>
        Our developers are all product-oriented, and our designers are all
        programmers.
      </p>

      <p>
        We work with a various technologies but our most common tech stacks are
        in the JavaScript and Ruby on Rails ecosystems.
      </p>

      <p>
        For all the roles below, I worked as a full-stack individual technical
        contributor and consultant, providing technical expertise for multiple
        clients ranging from start-ups to established growth-stage companies. I
        primarily contributed to clients' business success by delivering quality
        application code daily and by improving engineering team processes.
      </p>

      <div class={someDetailJobContainerStyle}>
        <h4>Ignite Development Team Lead</h4>
        <RoleDetails
          from={"Jan 2021"}
          to={"Now"}
          duration={"8 mos"}
          location={"Remote, (GMT-4)"}
        />
        <p>
          Development Team Lead for the ignite team specializing in early-stage
          proof of concepts and first product iterations. My role breaks down as
          70% individual contributor and 30% managerial.
        </p>
        <ul class="mt-4">
          <li>
            Work on client projects as a technical contributor and project
            manager
          </li>
          <li>
            Work as an advisor to other projects where I facilitate client
            meetings and help to keep the project on schedule and budget
          </li>
          <li>
            Work closely with the managing director to scope new projects and
            create project proposals for new clients
          </li>
          <li>Established technical strategy for the business unit</li>
          <li>
            Managed other developers, including weekly 1-on-1s, reviewing for
            promotions, and providing general support
          </li>
          <li>
            Established hiring strategy and technical interview process for new
            developers on the Ignite team
          </li>
        </ul>
      </div>

      <div class={someDetailJobContainerStyle}>
        <h4>Development Team Lead</h4>
        <RoleDetails
          from={"Mar 2020"}
          to={"Jan 2021"}
          duration={"8 mos"}
          location={"Boston, MA"}
        />
        <p>
          Development Team Lead for the Boston studio. The role broke down as
          90% individual contributor and 10% managerial.
        </p>
        <ul class="mt-4">
          <li>
            Work on client projects as a technical contributor and project
            manager
          </li>
          <li>
            Worked closely with the Managing Director to staff projects
            appropriately, maintain team member fulfillment, and define the
            direction for the studio.
          </li>
          <li>
            Managed other developers, providing technical and interpersonal
            support and guidance, having weekly 1-on-1s, and reviewing for
            promotions
          </li>
          <li>Responsible for interviewing new developers for the team</li>
        </ul>
      </div>

      <div class={someDetailJobContainerStyle}>
        <h4>Senior Developer</h4>
        <RoleDetails
          from={"Sep 2019"}
          to={"Mar 2020"}
          duration={"6 mos"}
          location={"Boston, MA"}
        />
        <p class="mt-4">
          Senior Developer in the Boston studio. My role was primarily ensuring
          the technical success of client projects.
        </p>
        <ul class="mt-4">
          <li>Expected to lead and work solo on client projects</li>
          <li>
            Worked with product owners to establish product roadmaps, especially
            with input from a technical perspective
          </li>
          <li>
            Brokedown high-level business goals into shippable product
            iterations
          </li>
          <li>
            Worked with other product developers and designers to build and ship
            new product features
          </li>
          <li>
            Mentored junior engineers on client teams on best practices,
            including design patterns and test driven development
          </li>
        </ul>
      </div>

      <div class={someDetailJobContainerStyle}>
        <h4>Developer</h4>
        <RoleDetails
          from={"Jun 2018"}
          to={"Sep 2019"}
          duration={"1 yr 4 mos"}
          location={"Boston, MA"}
        />
        <p class="mt-4">
          Developer at thoughtbot. I worked closely with clients on various
          projects as a technical contributor helping to make the project more
          successful.
        </p>
        <ul class="mt-4">
          <li>
            Brokedown high-level business goals into shippable product
            iterations
          </li>
          <li>
            Worked closely with designers and product managers to implement and
            ship successful product solutions for users and clients.
          </li>
          <li>
            Implemented best practices for software delivery, including
            continuous integration and continuous deployment
          </li>
          <li>
            Helped clients identify potential technical solutions for new
            products and improve existing applications by addressing technical
            debt
          </li>
          <li>
            Improved application performance by identifying and addressing non
            performant code and database queries
          </li>
          <li>
            Contributed to the technology community by giving technical
            presentations, writing blog posts, and organizing for the Boston
            Ruby, Elixir, and Scala Meetup groups.
          </li>
        </ul>
      </div>

      <div class={someDetailJobContainerStyle}>
        <h4>Apprentice</h4>
        <RoleDetails
          from={"Mar 2018"}
          to={"Jun 2018"}
          duration={"3 mos"}
          location={"Boston, MA"}
        />
        <p>
          Apprentice at thoughtbot. Apprentices work closely with thoughtbot
          developer mentors on client projects to develop the required skill set
          to be billable. I focused on learning the advanced patterns and
          practices used by thoughtbot developers to ensure quality and reliable
          software and software delivery methods.
        </p>

        <h5 class="mt-4 mb-2">Skills focused on:</h5>
        <ul>
          <li>Test-Driven Development</li>
          <li>Production software patterns</li>
          <li>TypeScript</li>
        </ul>
      </div>
    </div>

    <div class="mt-12">
      <h3>DeSimone Consulting Engineers</h3>
      <div>2 years, 11 months</div>

      <div class={someDetailJobContainerStyle}>
        <h4>Project Engineer</h4>
        <RoleDetails
          from={"Aug 2016"}
          to={"July 2017"}
          duration={"1 yr"}
          location={"Boston, MA"}
        />

        <p class="mt-4">
          Created and maintained software programs used by other engineers to
          automate the analysis of wind design loads, design of long-span
          structural members, design high-strength connections compliant with
          the California Building Code.
        </p>

        <p class="mt-4">
          Responsible for design solutions and analysis used to produce
          construction documents for building structural systems. Projects are
          primarily in the commercial sector.
        </p>
      </div>

      <div class={someDetailJobContainerStyle}>
        <h4>Project Engineer</h4>
        <RoleDetails
          from={"Sep 2014"}
          to={"Aug 2016"}
          duration={"2 yrs"}
          location={"Las Vegas, NV"}
        />
        <p class="mt-4">
          Responsible for design solutions and analysis used to produce
          construction documents for building structural systems. Projects are
          primarily in the hospitality sector and included The Surf Club in
          Miami by architect Richard Meiers and the Resorts World Las Vegas
          project by Gensler.
        </p>
      </div>
    </div>

    <div class="mt-12">
      <h3>Lionakis</h3>
      <div>9 months</div>

      <div class={someDetailJobContainerStyle}>
        <h4>Designer/Project Engineer</h4>
        <RoleDetails
          from={"Jan 2014"}
          to={"Aug 2016"}
          duration={"9 mos"}
          location={"Sacramento, CA"}
        />
        <p class="mt-4">
          Responsible for design solutions and analysis used to produce
          construction documents for building structural systems. Projects are
          primarily educational and healthcare facilities. Performed analysis of
          existing and new structures for code and structural compliance.
        </p>
      </div>
    </div>
  </div>

  <div class="mt-24">
    <h2>in more detail</h2>
    <div>individual contributions and full timeline</div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"May 2021"} to={"Now"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Fraiser</h3>
        <div class="mb-2 sub">Client Project</div>
        <TechnologyList
          list={[
            "Svelte",
            "InertiaJS",
            "TypeScript",
            "Ruby on Rails",
            "Tailwindcss",
            "Postgres",
            "Heroku",
            "S3",
          ]}
        />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Mar 2021"} to={"May 2021"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Tradewinds</h3>
        <div class="sub">Client Project</div>
        <TechnologyList
          list={[
            "StimulusJS",
            "TypeScript",
            "Ruby on Rails",
            "Tailwindcss",
            "LeafletJS",
            "Postgres",
            "Heroku",
          ]}
        />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Jan 2021"} to={"Mar 2021"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Epic Games: Houseparty</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["React", "TypeScript", "StyledComponents"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Jan 2021"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>thoughtbot Ignite - Development Team Lead</h3>
        <div class="sub">Promotion</div>
        <p>
          In January 2021, thoughtbot restructures from city-based studios to
          service offering based studios. I joined the Ignite team, specializing
          in the earliest stage products, continuing as a Development Team Lead.
          I am the most senior developer on the team. My duties expanded to
          include establishing the technical strategy for the team and the
          hiring strategy for new developers.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"May 2020"} to={"Jan 2021"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>PathCheck</h3>
        <div class="sub">Client Project</div>
        <TechnologyList
          list={[
            "React Native",
            "TypeScript",
            "Native APIs",
            "Swift",
            "Fastlane",
          ]}
        />
        <div>
          <a class="lnk" href="https://thoughtbot.com/case-studies/pathcheck"
            >Case Study</a
          >
          <a class="lnk" href="https://github.com/Path-Check/gaen-mobile"
            >github</a
          >
        </div>
        <p class="mt-2">
          I was a core contributor and technical project manager for PathCheck's
          Covid-19 contact tracing consumer app project. Over a few months, we
          designed, developed, and shipped the application to 7 states and
          countries, which had millions of downloads and helped fight Covid-19.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Mar 2020"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>thoughtbot Development Team Lead</h3>
        <div class="sub">Promotion</div>
        <p>
          In Q1 of 2020, thoughtbot introduces the Development Team Lead
          position. I am included among the initial group of developers to
          receive the promotion. The role continues to have four days a week of
          working on client projects as an individual contributor and adds the
          new responsibility of directly managing 2 to 3 other developers.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Feb 2020"} to={"May 2020"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>HomeMinder Mobile MVP</h3>
        <div class="sub">Client Project</div>
        <TechnologyList
          list={[
            "PWA",
            "Elm",
            "Capacitor",
            "Tailwindcss",
            "NodeJS",
            "Postgres",
            "Netlify",
          ]}
        />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Feb 2020"} to={"May 2020"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>thoughtbot Senior Developer</h3>
        <div class="sub">Promotion</div>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Aug 2020"} to={"Jan 2020"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>AiRXOS Mobile Applications</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["React Native", "TypeScript", "Redux"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Apr 2019"} to={"Aug 2019"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Careport</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["Ruby on Rails", "JQuery", "SCSS"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Now 2018"} to={"Mar 2019"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>ComplianceMate</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["Ruby on Rails", "AWS", "Postgres"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Oct 2018"} to={"Nov 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Splitfit</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["React Native"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Jun 2018"} to={"Sep 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Careport</h3>
        <div class="sub">Client Project</div>
        <TechnologyList list={["Ruby on Rails", "Elm", "Postgres"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Feb 2020"} to={"May 2020"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>thoughtbot Developer</h3>
        <div class="sub">Promotion</div>
        <p>
          After completing my apprenticeship, I was offered a developer role at
          thoughtbot in their Boston studio.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"May 2018"} to={"Jun 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Mass Challenge</h3>
        <div class="sub">Apprenticeship - Month 3</div>
        <TechnologyList list={["ReactJS", "TypeScript"]} />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Apr 2018"} to={"May 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>JoyDrive</h3>
        <div class="sub">Apprenticeship - Month 2</div>
        <TechnologyList list={["Pheonix", "Elixir"]} />
        <p />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Mar 2018"} to={"Apr 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Careport</h3>
        <div class="sub">Apprenticeship - Month 1</div>
        <TechnologyList list={["Ruby on Rails", "Elm", "Heroku"]} />
        <p />
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Feb 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>thoughtbot Apprenticeship</h3>
        <p>
          I applied to thoughtbot as a developer and received an offer for their
          apprenticeship program. The thoughtbot apprenticeship is a three-month
          paid program offered to newer developers to help build out their
          technical and consulting skillsets. While an apprentice, I learned
          best practices from senior thoughtbot developers, including
          test-driven development, vim, team management, and software delivery
          methods.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Sep 2017"} to={"Feb 2018"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Independent Contracting</h3>
        <TechnologyList list={["React Native"]} />
        <p>
          After graduating App Academy, I started my job search and worked as a
          part-time independent contractor for a start-up helping build out
          their React Native application.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Fall 2017"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>App Academy</h3>
        <div>San Francisco, CA</div>

        <p>
          A highly selective and rigorous software engineering program focusing
          on Ruby on Rails and ReactJS web development.
        </p>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Aug 2016"} to={"July 2017"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Desimone Consulting Engineers</h3>
        <div>Boston, MA</div>
        <ul class="mt-4">
          <li>
            Designed structural gravity and lateral systems to meet
            architectural and code requirements
          </li>

          <li>
            Responded to and made recommendations for RFI's and urgent
            change-orders
          </li>

          <li>
            Designed connection and member details for concrete and wood
            construction
          </li>

          <li>
            Reviewed and made recommendations on documents by other
            professionals including post-tensioned concrete reinforcement, mild
            reinforcement concrete, and steel connections
          </li>
        </ul>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Sep 2014"} to={"Aug 2016"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Desimone Consulting Engineers</h3>
        <div>Las Vegas, NV</div>
        <ul class="mt-4">
          <li>
            Designed structural gravity and lateral systems to meet
            architectural and code requirements
          </li>

          <li>
            Responded to and made recommendations for RFI's and urgent
            change-orders
          </li>

          <li>
            Designed connection and member details for concrete and wood
            construction
          </li>

          <li>
            Reviewed and made recommendations on documents by other
            professionals, including post-tensioned concrete reinforcement, mild
            reinforcement concrete, and steel connections
          </li>
        </ul>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"Jul 2014"} to={"Aug 2014"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>Lionakis</h3>
        <div>Sacramento, CA</div>
        <ul class="mt-4">
          <li>
            Selected and designed structural lateral and gravity systems to meet
            architectural and code requirements
          </li>
          <li>
            Reviewed and made recommendations on documents by other
            professionals
          </li>
          <li>Reviewed and approved shop drawings for steel elements</li>
          <li>
            Utilized drafting skills for wood, steel, and concrete construction
            document development
          </li>
          <li>Responded to review agency comments.</li>
          <li>Managed project deadlines</li>
        </ul>
      </div>
    </div>

    <div class={timelineListItemContainerStyle}>
      <div class={timelineLeftContainerStyle}>
        <DateRange from={"2008"} to={"2013"} />
      </div>
      <div class={timelineRightContainerStyle}>
        <h3>School</h3>
        <p>California Polytech University</p>
        <p>San Luis Obispo, CA</p>
        <p>Architectural Engineering Major</p>
        <p>Mathematics Minor</p>
        <p>Cum Laude</p>
      </div>
    </div>
  </div>
</div>
