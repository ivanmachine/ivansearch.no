<script>
  let query = "";

  function handleSearch() {
    alert(query);
  }
</script>

<div
  class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-r from-red-500 via-yellow-500 to-red-500"
>
  <div class="text-6xl font-bold text-red-700 flex items-center mb-8">
    IvanSearch
    <span class="ml-4 text-4xl">☭</span>
    <!-- Communism symbol -->
  </div>

  <div class="flex items-center space-x-4">
    <input
      type="text"
      bind:value={query}
      placeholder="Search..."
      class="px-4 py-2 text-lg border-2 border-red-700 rounded-md focus:outline-none focus:ring-2 focus:ring-red-500"
    />
    <button
      on:click={handleSearch}
      class="px-6 py-2 text-lg font-semibold text-white bg-red-700 rounded-md hover:bg-red-800 focus:outline-none focus:ring-2 focus:ring-red-500"
    >
      Search
    </button>
  </div>
</div>

<style>
  /* Add any custom styles here if needed */
</style>
