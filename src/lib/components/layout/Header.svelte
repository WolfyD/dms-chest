<script lang="ts">
    import { page } from '$app/stores';
</script>

<div class="app-header-title">
    <h1>Welcome to the DM's Chest</h1>
    {#if $page.url.pathname !== '/'}
        <div class="home-container">
            <a href="/"><i class="ri-home-fill"></i> Home</a>
        </div>
    {/if}
</div>
