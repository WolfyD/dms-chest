<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import '$lib/styles/tabs.css';

    function handleTabClick(tabId: string) {
        activeTab.set(tabId);
    }

    function handleKeydown(event: KeyboardEvent, tabId: string) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            handleTabClick(tabId);
        }
    }
</script>

<div class="tabs-container">
    {#each Object.entries(categories) as [id, category]}
        <div 
            class="tab {id}" 
            class:selected={$activeTab === id} 
            on:click={() => handleTabClick(id)} 
            on:keydown={(e) => handleKeydown(e, id)} 
            role="tab" 
            tabindex="0"
        >
            <i class={category.icon}></i> 
            <span class="category-name">{category.name}</span>
            <span class="tool-count">{category.tools.length} tools</span>
        </div>
    {/each}
</div>