<script lang="ts">
    import { toolData } from '$lib/stores/toolData';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import '$lib/styles/tool-grid.css';
    import '$lib/styles/components.css';
    import '$lib/styles/tool-box-item.css';
    import { getLocationParents, getLocationChildren, getLocations } from '$lib/utils/region';
    import AutocompleteInput from '$lib/components/AutocompleteInput.svelte';

    let suggestions: { name: string, id: number }[] = [];
    let topLevelRegionName = '';
    let topLevelRegionId = 0;
    let locationList: { name: string, id: number, type: string }[] = [];
    function handleSubmit() {
        console.log('submit');
    }

    function handleRegionNameSelect(item: { id: number | string, label: string, data: any }) {
        topLevelRegionName = item.label;
        topLevelRegionId = Number(item.id);

        if(topLevelRegionId != 0) {
            let locationParents = getLocationList(topLevelRegionId);      
            let locationChildren = getLocationList(topLevelRegionId, true);
            console.log("locationParents", locationParents);
            console.log("locationChildren", locationChildren);
        }
    }

    async function getLocationList(id: number, children: boolean = false) : Promise<{ name: string, id: number, type: string }[]> {
        const locations = await (children ? getLocationChildren : getLocationParents)(id);
        console.log("locations", locations);
        if(locations.length > 0) {
            return locations;
        }
        return [];
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
                <AutocompleteInput
                    searchFn={getLocations}
                    bind:value={topLevelRegionName}
                    placeholder="Enter location name..."
                    onSelect={handleRegionNameSelect}
                />
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

