<script lang="ts">
  import "../app.css";
  import Header from "$lib/components/nav/Header.svelte";
  let { children } = $props();
</script>

<Header />

<div
  class="min-h-screen bg-gradient-to-r from-red-500 via-yellow-500 to-red-500"
>
  {@render children()}
</div>

<style>
  /* Add any custom styles here if needed */
</style>
