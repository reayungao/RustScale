<script>
  import "../app.css";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Toast from "$lib/components/Toast.svelte";
</script>

<div
  class="flex h-screen bg-[var(--bg-canvas)] text-[var(--text-primary)] overflow-hidden font-sans transition-colors duration-200"
>
  <Sidebar />

  <main class="flex-1 relative overflow-hidden">
    <slot />
  </main>

  <Toast />
</div>
