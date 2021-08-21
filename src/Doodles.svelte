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
</script>

<div class="w-full">
  <h1 class="h1">Doodles</h1>
  <p class="mt-2 text-sm text-gray-700">
    a collection of programming 'doodles'
  </p>

  <ul class="mt-8">
    {#each doodles as { url, label }}
      <li>
        <a class="mt-4 text-2xl lnk" href={url}>{label}</a>
      </li>
    {/each}
  </ul>
</div>
