<script lang="ts">
  import { Link } from "svelte-routing"

  export let to = ""

  let isActive: boolean = false

  function getProps({ location, href, isPartiallyCurrent, isCurrent }) {
    isActive = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent;
    return {}
  }

  const mobileBaseStyle = "items-center text-lg font-medium py-2 px-4 border-l-4 block"
  const desktopBaseStyle = "sm:inline-flex sm:border-b-4 sm:border-l-0 sm:px-2 sm:pt-2"
  const baseStyle = `${mobileBaseStyle} ${desktopBaseStyle}`

  const mobileActiveStyle = "text-indigo-700 border-indigo-500 bg-indigo-50"
  const desktopActiveStyle = "sm:text-gray-900 sm:bg-white"
  const activeStyle = `${mobileActiveStyle} ${desktopActiveStyle}`

  const mobileInactiveStyle = "text-gray-500 border-transparent hover:border-gray-300 hover:text-gray-700 hover:bg-gray-50"
  const desktopInactiveStyle = "sm:hover:bg-white"
  const inactiveStyle = `${mobileInactiveStyle} ${desktopInactiveStyle}`

  $: style = isActive ?  `${baseStyle} ${activeStyle}` : `${baseStyle}
  ${inactiveStyle}`
</script>

<Link to="{to}" class="{style}" getProps="{getProps}">
  <slot />
</Link>
