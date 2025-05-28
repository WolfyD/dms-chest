<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  // This layout will be applied to all pages
  import '$lib/styles/global.css';
  import '$lib/styles/tabs.css';
  import Footer from '$lib/components/layout/Footer.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import { initHeightAdjustment } from '$lib/utils/ui';
  import { initializeDatabase, closeDatabase } from '$lib/database/init';

  let cleanup: (() => void) | undefined;

  onMount(() => {
    // Initialize database
    initializeDatabase().catch(error => {
      console.error('Failed to initialize database:', error);
    });

    // Initialize height adjustment
    cleanup = initHeightAdjustment();

    // Return cleanup function
    return () => {
      cleanup?.();
    };
  });

  onDestroy(async () => {
    try {
      await closeDatabase();
    } catch (error) {
      console.error('Failed to close database:', error);
    }
  });
</script>

<div class="app">
  <!-- Header -->
  <Header />
  
  <main>
    <!-- Page Content -->
    <slot />
  </main>
  
  <!-- Footer -->
  <div class="footer-container">
    <Footer />
  </div>
</div> 
