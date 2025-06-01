<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import { get_id_from_string } from '$lib/utils/string';
    import { openTool } from '$lib/utils/ui';
    import '$lib/styles/tool-grid.css';

    $: currentCategory = categories[$activeTab];
</script>

<div class="tool-grid">
    {#if currentCategory}
        <h2>{currentCategory.name} Tools</h2>
        <p>{currentCategory.description}</p>
        <div class="tools-container">
            {#each currentCategory.tools as [tool, description, longDescription]}
                <button class="tool-card" id={get_id_from_string(tool)} data-tool={tool} onclick={() => openTool(get_id_from_string(tool), longDescription)} tabindex="0">
                    <h3>{tool}</h3>
                    <p>{description}</p>
                </button>
            {/each}
        </div>
    {/if}
</div>
