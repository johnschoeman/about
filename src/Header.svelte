<script lang="ts">
  import NavLink from "./NavLink.svelte"

  let isOpen: boolean = false

  const handleOnClick = () => {
    isOpen = !isOpen
  }
</script>

<nav class="mb-12 sm:mb-24">
  <div class="max-w-4xl px-8 mx-auto">
    <div class="flex items-center justify-between h-24">
      <h1 class="text-lg font-black"><a href="/">john schoeman</a></h1>
      <div class="flex">
        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
          <NavLink to="/">
            home
          </NavLink>
          <NavLink to="/about" >
            about
          </NavLink>
        </div>
      </div>

     <div class="flex items-center -mr-2 sm:hidden">
        <!-- Mobile menu button -->
        <button
          type="button"
          on:click={handleOnClick}
          class="inline-flex items-center justify-center p-2 text-gray-400 rounded-md hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
          aria-controls="mobile-menu"
          aria-expanded="false"
          >
          <span class="sr-only">Open main menu</span>
          <!--
            Icon when menu is closed.

            Heroicon name: outline/menu

            Menu open: "hidden", Menu closed: "block"
          -->
          <svg class="block w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
          <!--
            Icon when menu is open.

            Heroicon name: outline/x

            Menu open: "block", Menu closed: "hidden"
          -->
          <svg class="hidden w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  </div>

  <!-- Mobile menu, show/hide based on menu state. -->
  {#if isOpen}
  <div class="sm:hidden" id="mobile-menu">
    <div class="pt-2 pb-3">
      <NavLink to="/">
        home
      </NavLink>
      <NavLink to="/about">
        about
      </NavLink>
    </div>
  </div>
  {/if}
</nav>
