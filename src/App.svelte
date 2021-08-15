<script lang="ts">
  import { Router, Route } from "svelte-routing"

  import Home from "./Home.svelte"
  import Header from "./Header.svelte"
  import About from "./About.svelte"
  import Doodles from "./Doodles.svelte"
  import WorkHistory from "./WorkHistory.svelte"
  import Footer from "./Footer.svelte"

  export let url = ""
</script>

<main class="flex flex-col justify-between h-screen">
  <Router bind:url>
    <Header />

    <div class="max-w-4xl px-8 pb-24 mx-auto mb-auto">
      <Route path="/doodles"><Doodles /></Route>
      <Route path="/about"><About /></Route>
      <Route path="/work_history"><WorkHistory /></Route>
      <Route path="/"><Home /></Route>
    </div>

    <Footer />
  </Router>
</main>

<style global lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  @layer components {
    .lnk {
      @apply inline-block text-blue-600 underline;
    }
    .lnk:visited {
      @apply text-indigo-600;
    }
    .lnk:hover {
      @apply text-blue-800;
    }
    .lnk:focus {
      @apply outline-none;
    }

    .h1 {
      @apply text-6xl font-black text-gray-900;
    }
    .h2 {
      @apply text-4xl font-black text-gray-900;
    }
    .h3 {
      @apply text-2xl font-semibold text-gray-900;
    }
    .p {
      @apply text-gray-900;
    }
    .sub {
      @apply text-sm text-gray-700;
    }

    .lnk-black {
      @apply text-gray-800 text-base;
    }
    .lnk-black:visited {
      @apply text-gray-600;
    }

    /* Markdown Start */
    /* Additional vertical padding used by kbd tag. */
    .py-05 {
      padding-top: 0.125rem;
      padding-bottom: 0.125rem;
    }

    .markdown {
      @apply text-gray-900 leading-normal break-words;
    }

    .markdown p {
      @apply mt-2;
    }

    .markdown > * + * {
      @apply mt-0;
    }

    .markdown li + li {
      @apply mt-1;
    }

    .markdown li > p + p {
      @apply mt-8;
    }

    .markdown strong {
      @apply font-semibold;
    }

    .markdown a {
      @apply text-blue-600 font-semibold;
    }

    .markdown strong a {
      @apply font-bold;
    }

    .markdown h1 {
      @apply leading-tight text-6xl font-black text-gray-900;
    }

    .markdown h2 {
      @apply leading-tight text-4xl font-black text-gray-900;
    }

    .markdown h3 {
      @apply text-2xl font-semibold text-gray-900;
    }

    .markdown h4 {
      @apply leading-tight text-base font-semibold text-gray-900;
    }

    .markdown h5 {
      @apply text-sm font-semibold text-gray-900;
    }

    .markdown h6 {
      @apply leading-tight text-sm font-semibold text-gray-600;
    }

    .markdown blockquote {
      @apply text-base border-l-4 border-gray-300 pl-4 pr-4 text-gray-600;
    }

    .markdown code {
      @apply font-mono text-sm inline bg-gray-200 rounded px-1 py-05;
    }

    .markdown pre {
      @apply bg-gray-100 rounded p-4;
    }

    .markdown pre code {
      @apply block bg-transparent p-0 overflow-visible rounded-none;
    }

    .markdown ul {
      @apply text-base pl-8 list-disc;
    }

    .markdown ol {
      @apply text-base pl-8 list-decimal;
    }

    .markdown kbd {
      @apply text-xs inline-block rounded border px-1 py-05 align-middle font-normal font-mono shadow;
    }

    .markdown table {
      @apply text-base border-gray-600;
    }

    .markdown th {
      @apply border py-1 px-3;
    }

    .markdown td {
      @apply border py-1 px-3;
    }

    /* Override pygments style background color. */
    .markdown .highlight pre {
      @apply bg-gray-100 !important;
    }

    /* Markdown End */
  }
</style>
