<script>
  import { page } from "$app/stores";
  import MainPage from "$lib/components/search/MainPage.svelte";
  import ResultsPage from "$lib/components/search/ResultsPage.svelte";

  $: query = $page.url.searchParams.get("query");

  function handleSearch() {
    alert(query);
  }
</script>

{#if query === null}
  <MainPage />
{:else}
  <ResultsPage />
{/if}
