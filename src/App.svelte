<script lang="ts">
  import { Router, Route } from "svelte-routing"

  import Home from "./Home.svelte"
  import Header from "./Header.svelte"
  import About from "./About.svelte"
  import Doodles from "./Doodles.svelte"
  import Footer from "./Footer.svelte"

  export let url = ""
</script>

<main class="flex flex-col justify-between h-screen">
  <Router bind:url>
    <Header />

    <div class="max-w-4xl px-8 pb-24 mx-auto mb-auto">
      <Route path="/doodles"><Doodles /></Route>
      <Route path="/about"><About /></Route>
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
      @apply text-6xl font-black text-gray-900
    }
    .h2 {
      @apply text-4xl font-semibold text-gray-900
    }
    .p {
      @apply text-gray-900
    }
    .sub {
      @apply text-sm text-gray-700
    }

    .lnk-black {
      @apply text-gray-800 text-base;
    }
    .lnk-black:visited {
      @apply text-gray-600;
    }
  }
</style>
