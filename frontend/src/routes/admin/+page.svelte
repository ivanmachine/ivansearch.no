<!-- frontend/src/routes/admin/+page.svelte -->
<script>
  import { onMount } from "svelte";

  onMount(() => {
    const cookies = document.cookie.split("; ").reduce((acc, cookie) => {
      const [key, value] = cookie.split("=");
      acc[key] = value;
      return acc;
    }, {});

    if (cookies.auth !== "true") {
      window.location.href = "/login";
    }
  });
</script>

<h1>Admin panel</h1>
<!-- ... existing code ... -->
