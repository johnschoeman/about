<script lang="ts">
  import { Link } from "svelte-routing"
</script>

<nav class="flex flex-row w-full">
  <h1><a href="/">John Schoeman</a></h1>

  <ul class="flex flex-row">
    <Link to="/">Home</Link>
    <Link to="/doodles">Doodles</Link>
    <Link to="/about">About</Link>
  </ul>
</nav>
