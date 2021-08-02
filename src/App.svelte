<script lang="ts">
  import Home from "./Home.svelte"
</script>

<main>
  <Home />
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

