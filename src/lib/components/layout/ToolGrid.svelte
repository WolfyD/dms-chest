<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import '$lib/styles/tool-grid.css';

    $: currentCategory = categories[$activeTab];
</script>

<div class="tool-grid">
    {#if currentCategory}
        <h2>{currentCategory.name} Tools</h2>
        <p>{currentCategory.description}</p>
        <div class="tools-container">
            {#each currentCategory.tools as [tool, description]}
                <div class="tool-card" role="button" tabindex="0">
                    <h3>{tool}</h3>
                    <p>{description}</p>
                </div>
            {/each}
        </div>
    {/if}
</div>
