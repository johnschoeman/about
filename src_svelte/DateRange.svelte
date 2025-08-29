<script lang="ts">
  export let from: string
  export let to: string | null
</script>

<div class="w-full text-sm text-gray-700 sm:text-right leading-10">
  {`${from}${to ? ` - ${to}` : ""}`}
</div>
