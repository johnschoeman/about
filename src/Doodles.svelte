<script lang="ts">
  const doodles = [
    {
      label: "doodles.camp 🏕️",
      url: "https://www.doodles.camp/",
    },
    {
      label: "doodles.kitchen 🍳",
      url: "https://www.doodles.kitchen/",

    }
  ]

  const showIdx = (idx: number): string => {
    return idx.toString().padStart(2, "0")
  }
</script>

<h1 class="h1">doodles</h1>
<p class="mt-2 text-sm text-gray-700">a collection of programming 'doodles'</p>

<ul class="mt-8">
  {#each doodles as { url, label }, idx}
    <li class="flex flex-row items-center mt-8">
      <p class="w-12 h-full text-2xl font-bold text-gray-400 align-middle">
        {showIdx(idx)}
      </p>
      <a class="text-2xl align-middle lnk" href={url}>{label}</a>
    </li>
  {/each}
</ul>
