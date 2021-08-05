<script lang="ts">
  import { Router, Route } from "svelte-routing"

  import Home from "./Home.svelte"
  import Header from "./Header.svelte"
  import About from "./About.svelte"
  import Doodles from "./Doodles.svelte"
  import Footer from "./Footer.svelte"

  export let url = ""
</script>

<main>
<div class="max-w-full px-12 py-12 mx-auto bg-white">
  <div class="max-w-lg mx-auto">
  <Router bind:url>
    <Header />

    <div>
      <Route path="/doodles"><Doodles /></Route>
      <Route path="/about"><About /></Route>
      <Route path="/"><Home /></Route>
    </div>
  </Router>

  <Footer />
  </div>
</div>
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

  .lnk-black {
    @apply text-gray-800 text-base;
  }
  .lnk-black:visited {
    @apply text-gray-600;
  }
}
</style>

