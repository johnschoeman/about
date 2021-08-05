<script lang="ts">
  import { Router, Link, Route } from "svelte-routing"

  import Home from "./Home.svelte"
  import About from "./About.svelte"
  import Doodles from "./Doodles.svelte"
  import Footer from "./Footer.svelte"

  export let url = ""
</script>

<main>
  <Router bind:url>
    <nav>
      <h1><a href="/">John Schoeman</a></h1>

      <ul>
        <Link to="/">Home</Link>
        <Link to="/doodles">Doodles</Link>
        <Link to="/about">About</Link>
      </ul>
    </nav>

    <div>
      <Route path="/doodles"><Doodles /></Route>
      <Route path="/about"><About /></Route>
      <Route path="/"><Home /></Route>
    </div>
  </Router>

  <Footer />
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

