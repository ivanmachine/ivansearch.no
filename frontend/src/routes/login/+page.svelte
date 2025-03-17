<!-- frontend/src/routes/login/+page.svelte -->
<script lang="ts">
  import Cookies from "js-cookie";
  $: hasCookie = !!Cookies.get("admin_session");
  $: password = "";
  $: answer = "";

  async function handleLogout() {
    Cookies.remove("admin_session");
    hasCookie = false;
  }

  async function handleLogin() {
    try {
      const response = await fetch("/api/auth", {
        method: "GET",
        headers: {
          password: password,
        },
      });
      switch (response.status) {
        case 200:
          const cookie = response.headers.get("Cookie");
          if (cookie) {
            Cookies.set("admin_session", cookie, { expires: 7 });
            window.location.href = "/admin";
          } else {
            alert("Got 200 with no cookie, code better in the future");
          }
          break;
        case 401:
          answer = "Incorrect password";
          break;
        case 500:
          answer = "Error while logging in, propably your cookie is shit";
          break;
      }
    } catch (error) {
      console.error("Error while logging in: ", error);
      answer = "Error while logging in";
    }
  }
</script>

<div
  class="flex flex-col items-center justify-center min-h-screen bg-gray-100 px-4"
>
  <div class="w-full max-w-md p-8 space-y-6 bg-white rounded-lg shadow-md">
    <h1 class="text-3xl font-bold text-center text-gray-800">Login</h1>
    {#if hasCookie}
      <button
        on:click={handleLogout}
        class="w-full py-2 px-4 bg-red-600 text-white font-medium rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 transition-colors"
      >
        Log Out
      </button>
    {:else}
      <form on:submit|preventDefault={handleLogin} class="space-y-4">
        <div>
          <input
            bind:value={password}
            placeholder="Enter password"
            class="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <button
          type="submit"
          class="w-full py-2 px-4 bg-blue-600 text-white font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
        >
          Login
        </button>
      </form>
    {/if}

    {#if answer}
      <p class="text-red-500 text-center">{answer}</p>
    {/if}
  </div>
</div>
