<script lang="ts">
  const doodles = [
    {
      label: "black sheep",
      url: "https://www.doodles.camp/#/doodles/black_sheep",
    },
    {
      label: "lock puzzle",
      url: "https://www.doodles.camp/#/doodles/lock_puzzle",
    },
    {
      label: "mom",
      url: "https://www.doodles.camp/#/doodles/mom",
    },

    { label: "recaman 2", url: "https://www.doodles.camp/#/doodles/recaman" },
    { label: "squares", url: "https://www.doodles.camp/#/doodles/squares" },
    {
      label: "b pad",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/b_pad.html",
    },
    {
      label: "tic tac toe",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/tic_tac_toe.html",
    },
    {
      label: "recaman",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/recaman.html",
    },
    {
      label: "solar solar system 2",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/solar_system_2.html",
    },
    {
      label: "asteroids",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/asteroids.html",
    },
    {
      label: "tree",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/tree.html",
    },
    {
      label: "mandlebrot",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/mandlebrot.html",
    },
    {
      label: "koch curve",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/koch_curve.html",
    },
    {
      label: "mirror prime",
      url: "https://boring-hypatia-5625e0.netlify.app/doodles/mirror_prime.html",
    },
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
