<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import Footer from '$lib/components/layout/Footer.svelte';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import Main from '$lib/components/layout/Main.svelte';
  import AppLayout from '$lib/components/layout/AppLayout.svelte';

  // You can import components from your components directory
  // import Header from '$lib/components/layout/Header.svelte';
  // import Footer from '$lib/components/layout/Footer.svelte';
</script>

<AppLayout />
