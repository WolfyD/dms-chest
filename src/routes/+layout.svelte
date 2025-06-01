<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  // This layout will be applied to all pages
  import '$lib/styles/global.css';
  import '$lib/styles/tabs.css';
  import Footer from '$lib/components/layout/Footer.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import { initHeightAdjustment } from '$lib/utils/ui';
  import { initializeDatabase, closeDatabase } from '$lib/database/init';
  import { invoke } from '@tauri-apps/api/core';

  let cleanup: (() => void) | undefined;

  onMount(() => {
    const init = async () => {
      try {
        // Initialize database
        await initializeDatabase().catch(error => {
          console.error('Failed to initialize database:', error);
        });

        // Extract executables
        await invoke('extract_executables').catch(error => {
          console.error('Failed to extract executables:', error);
        });

        // Initialize height adjustment
        cleanup = initHeightAdjustment();
      } catch (error) {
        console.error('Failed to initialize:', error);
      }
    };

    init();

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
