<script lang="ts">
    import Tabs from './Tabs.svelte';
    import ToolGrid from './ToolGrid.svelte';
    import '$lib/styles/layout.css';
</script>

<div class="app-layout">
    <header class="app-header">
        <Tabs />
    </header>
    
    <main class="app-content">
        <aside class="sidebar">
            <!-- Sidebar content (e.g., filters, quick actions) -->
        </aside>
        
        <section class="main-content">
            <ToolGrid />
        </section>
    </main>
</div> 