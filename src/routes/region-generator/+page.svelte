<script lang="ts">
    import { toolData } from '$lib/stores/toolData';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import '$lib/styles/tool-grid.css';
    import '$lib/styles/components.css';
    import '$lib/styles/tool-box-item.css';
    import { getLocationParents, getLocations, createAutocomplete } from '$lib/utils/region';
    
    let suggestions: { name: string, id: number }[] = [];
    let topLevelRegionName = '';
    function handleSubmit() {
        console.log('submit');
    }

    async function handleRegionNameInputWithDebounceAndAutocomplete(event: Event) {
        const input = event.target as HTMLInputElement;
        topLevelRegionName = input.value;
        const debouncedSearch = debounce(getLocations, 300);
        const suggestions = await debouncedSearch(topLevelRegionName);
        console.log(suggestions);
    }

    function debounce(func: Function, wait: number) {
        let timeout: NodeJS.Timeout;
        return function executedFunction(...args: any[]) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    function handleSuggestionClick(suggestion: { name: string, id: number }) {
        topLevelRegionName = suggestion.name;
        suggestions = [];
    }

</script>

<div class="form-card tool-form region-generator">
    <h2>Region Generator</h2>
    <p>Creates unique regions complete with climate, terrain, settlements, and local flavor — fully customizable to suit your tone and geography.</p>

    

    <form on:submit|preventDefault={handleSubmit} class="world-form">
        <div class="form-section">
            <h3>Top Level Region</h3>
            <div class="form-group">
                <label for="regionName">Region Name</label>
                <input type="text" bind:value={topLevelRegionName} on:input={handleRegionNameInputWithDebounceAndAutocomplete} id="regionName" required />
                <div class="autocomplete-container">
                    <div class="suggestions-list">
                        {#each suggestions as suggestion}
                            <menuitem class="suggestion-item" on:click={() => handleSuggestionClick(suggestion)}>
                                {suggestion.name}
                            </menuitem>
                        {/each}
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label for="description">Description</label>
                <textarea class="description-textarea" id="description" rows="4"></textarea>
            </div>
        </div>

        

        <div class="form-section">
            <h3>World History & Culture</h3>
            <div class="form-group">
                <label for="history">History</label>
                <textarea id="history" rows="6"></textarea>
            </div>
            <div class="form-group">
                <label for="religions">Religions</label>
                <div class="array-input">
                    <input type="text" id="religions" placeholder="Add religion" />
                </div>
                <div class="array-list">
                    
                </div>
            </div>
            <div class="form-group">
                <label for="pantheon">Pantheon</label>
                <div class="array-input">
                    <input type="text" id="pantheon" placeholder="Add deity" />
                </div>
                <div class="array-list">
                   
                </div>
            </div>
        </div>

        <div class="form-section">
            <h3>Additional Information</h3>
            <div class="form-group">
                <label for="notableLandmarks">Notable Landmarks</label>
                <div class="array-input">
                    <input type="text" id="notableLandmarks" placeholder="Add landmark" />
                </div>
                <div class="array-list">
                    
                </div>
            </div>
            <div class="form-group">
                <label for="calendarInfo">Calendar Information</label>
                <textarea id="calendarInfo" rows="3"></textarea>
            </div>
            <div class="form-group">
                <label for="establishedMaterial">Established Material</label>
                <textarea id="establishedMaterial" rows="3"></textarea>
            </div>
        </div>

        <div class="form-actions">
            <button type="submit" class="primary-button">Create World</button>
        </div>
    </form>
</div>

