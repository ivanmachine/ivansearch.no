<script>
  import { page } from "$app/stores";
  $: query = $page.url.searchParams.get("query");
</script>

<h2>Showing results for: {query}</h2>
