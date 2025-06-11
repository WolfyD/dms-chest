<script lang="ts">
    import Tabs from './Tabs.svelte';
    import ToolGrid from './ToolGrid.svelte';
    // Import unified CSS
    import '$lib/styles/index.css';
</script>

<div class="app-layout">
    <!-- Left Sidebar with Tabs -->
    <aside class="app-sidebar">
        <Tabs />
    </aside>
    
    <!-- Main Content Area -->
    <main class="app-main">
        <ToolGrid />
    </main>
</div> 