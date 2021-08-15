<script lang="ts">
  export let from: string
  export let to: string
  export let duration: string
  export let location: string
</script>

<div>
  <div class="w-full text-sm text-gray-700">
    {from} - {to} • {duration}
  </div>
  <div class="text-sm text-gray-700">
    {location}
  </div>
</div>
