<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    // Styles now imported via unified CSS system
    import { generateWorldName, getWorldDescription } from '$lib/utils/tracery';
    import { makeSlightlyDifferent } from '$lib/utils/compromise';
    import { randomizeValues, getRandomPlane, getRandomSpecies } from '$lib/utils/world';

    let world_id = '';
    let tool_description = '';

    window.addEventListener('message', (event) => {
        const message = JSON.parse(event.data);
        if (message.type === 'world-generator') {
            // Handle the data
            world_id = message.data.world_id;
            tool_description = message.data.description;
        }
    });


    // Form data
    let worldName = '';
    let description = '';
    let genre = 'Fantasy';
    let tone = 'Mixed';
    let techLevel = 'Medieval';
    let magicLevel = 'Low';
    let dominantSpecies: string[] = [];
    let otherSpecies: string[] = [];
    let religions: string[] = [];
    let pantheon: string[] = [];
    let continents: string[] = [];
    let majorNations: string[] = [];
    let notableLandmarks: string[] = [];
    let history = '';
    let planarStructure = getRandomPlane();
    let calendarInfo = '';
    let establishedMaterial = '';

    // Location is connected to world through world_id so should be loaded with the world

    // Location data
    let locationName = '';
    let locationDescription = '';
    let locationType = 'Planet';
    let locationPopulation = 0;
    let locationKnownFor = '';
    let locationTerrain = '';
    let locationClimate = '';
    let locationDangerLevel = 0;
    let locationNotableLandmarks: string[] = [];
    let locationHistory = '';
    let locationMajorEvents: string[] = [];
    let locationNotes = '';
    let locationHasMap = false;
    let locationMapId:number | null = null;
    let locationMapImageUrl = '';
    let locationMapLocation: object = {};


    // Available options
    const genres = ['Fantasy', 'Sci-Fi', 'Post-Apocalyptic', 'Modern', 'Historical', 'Other'];
    const tones = ['Heroic', 'Dark', 'Grimdark', 'Hopeful', 'Tragic', 'Comedic', 'Mixed', 'Other'];
    const techLevels = ['Stone Age', 'Bronze Age', 'Iron Age', 'Medieval', 'Renaissance', 'Industrial', 'Modern', 'Near Future', 'Far Future', 'Magitech', 'Other'];
    const magicLevels = ['None', 'Low', 'Moderate', 'High', 'Wild', 'Divine Only', 'Unknown', 'Other'];

    // Location options
    const locationTypes = ['Planet', 'Moon', 'Continent', 'Landmass', 'Isle', 'Island', 'Region', 'Union', 
                            'Agglomeration', 'Country', 'Province', 'County', 'City', 'Town', 'Village', 
                            'Hamlet', 'District', 'Street', 'Road', 'Building', 'Room', 'Other'];

    // Form submission
    async function handleSubmit() {
        try {
            const worldData = {
                name: worldName,
                description,
                genre,
                tone,
                techLevel,
                magicLevel,
                dominantSpecies: JSON.stringify(dominantSpecies),
                otherSpecies: JSON.stringify(otherSpecies),
                religions: JSON.stringify(religions),
                pantheon: JSON.stringify(pantheon),
                continents: JSON.stringify(continents),
                majorNations: JSON.stringify(majorNations),
                notableLandmarks: JSON.stringify(notableLandmarks),
                history,
                planarStructure,
                calendarInfo,
                establishedMaterial
            };

            // Create location
            const locationData = {
                name: locationName,
                description: locationDescription,
                type: locationType,
                population: locationPopulation,
                knownFor: locationKnownFor,
                terrain: locationTerrain,
                climate: locationClimate,
                dangerLevel: locationDangerLevel,
                notableLandmarks: JSON.stringify(locationNotableLandmarks),
                history: locationHistory,
                majorEvents: JSON.stringify(locationMajorEvents),
                notes: locationNotes,
                hasMap: locationHasMap,
                mapId: locationMapId,
                mapImageUrl: locationMapImageUrl,
                mapLocation: locationMapLocation
            };

            resetForm();
        } catch (error) {
            console.error('Failed to create world:', error);
        }
    }

    function resetForm() {
        worldName = '';
        description = '';
        genre = 'Fantasy';
        tone = 'Mixed';
        techLevel = 'Medieval';
        magicLevel = 'Low';
        dominantSpecies = [];
        otherSpecies = [];
        religions = [];
        pantheon = [];
        continents = [];
        majorNations = [];
        notableLandmarks = [];
        history = '';
        planarStructure = 'Material Plane';
        calendarInfo = '';
        establishedMaterial = '';
    }

    // Helper functions for array fields
    function addToArray(array: string[], value: string) {
        if (value && !array.includes(value)) {
            array = [...array, value];
        }
        return array;
    }

    function removeFromArray(array: string[], value: string) {
        return array.filter(item => item !== value);
    }
</script>

<div class="form-card tool-form world-generator">
    <h2>World Generator</h2>
    {#if tool_description}
        <p>{tool_description}</p>
    {:else}
        <p>Create a new world with detailed settings and characteristics.</p>
    {/if}

    <button class="generator-button world-generator-button" type="button" on:click={async () => { 
        [genre, tone, techLevel, magicLevel] = randomizeValues(); 
        worldName = generateWorldName([genre, tone, techLevel, magicLevel]).name; 
        getWorldDescription([genre, tone, techLevel, magicLevel, worldName]).then(desc => description = desc); 
        planarStructure = getRandomPlane();
        [dominantSpecies, otherSpecies] = await getRandomSpecies().then(object => [object.dominantSpecies, object.otherSpecies]);
        // TODO: more to come
    }}>Generate New World</button>

    <form on:submit|preventDefault={handleSubmit} class="world-form">
        <div class="form-section">
            <h3>Basic Information</h3>
            <div class="form-group">
                <label for="worldName">World Name</label>
                <input type="text" id="worldName" bind:value={worldName} required /><button class="generator-button" type="button" on:click={() => worldName = generateWorldName([genre, tone, techLevel, magicLevel]).name}>Generate</button>
            </div>
            <div class="form-group">
                <label for="description">Description</label>
                <textarea class="description-textarea" id="description" bind:value={description} rows="4"></textarea><button class="generator-button world-description-generator" type="button" on:click={() => { if(!worldName){ worldName = generateWorldName([genre, tone, techLevel, magicLevel]).name } {getWorldDescription([genre, tone, techLevel, magicLevel, worldName]).then(desc => description = desc)} }}>Generate</button>
            </div>
        </div>

        <div class="form-section">
            <h3>World Settings</h3>
            <div class="form-group">
                <label for="genre">Genre</label>
                <select id="genre" bind:value={genre}>
                    {#each genres as genreOption}
                        <option value={genreOption}>{genreOption}</option>
                    {/each}
                </select>
            </div>
            <div class="form-group">
                <label for="tone">Tone</label>
                <select id="tone" bind:value={tone}>
                    {#each tones as toneOption}
                        <option value={toneOption}>{toneOption}</option>
                    {/each}
                </select>
            </div>
            <div class="form-group">
                <label for="techLevel">Technology Level</label>
                <select id="techLevel" bind:value={techLevel}>
                    {#each techLevels as techOption}
                        <option value={techOption}>{techOption}</option>
                    {/each}
                </select>
            </div>
            <div class="form-group">
                <label for="magicLevel">Magic Level</label>
                <select id="magicLevel" bind:value={magicLevel}>
                    {#each magicLevels as magicOption}
                        <option value={magicOption}>{magicOption}</option>
                    {/each}
                </select>
            </div>

            <button class="generator-button" type="button" on:click={() => { [genre, tone, techLevel, magicLevel] = randomizeValues() }}>Randomize</button>
        </div>

        <div class="form-section">
            <h3>World Inhabitants</h3>
            <div class="form-group">
                <label for="dominantSpecies">Dominant Species</label>
                <div class="array-input">
                    <input type="text" id="dominantSpecies" placeholder="Add dominant species" />
                    <button type="button" on:click={() => dominantSpecies = addToArray(dominantSpecies, (document.getElementById('dominantSpecies') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each dominantSpecies as species}
                        <span class="array-item">
                            {species}
                            <button type="button" on:click={() => dominantSpecies = removeFromArray(dominantSpecies, species)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
            <div class="form-group">
                <label for="otherSpecies">Other Species</label>
                <div class="array-input">
                    <input type="text" id="otherSpecies" placeholder="Add other species" />
                    <button type="button" on:click={() => otherSpecies = addToArray(otherSpecies, (document.getElementById('otherSpecies') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each otherSpecies as species}
                        <span class="array-item">
                            {species}
                            <button type="button" on:click={() => otherSpecies = removeFromArray(otherSpecies, species)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
        </div>

        <div class="form-section">
            <h3>World Structure</h3>
            <div class="form-group">
                <label for="planarStructure">Planar Structure</label>
                <input type="text" id="planarStructure" bind:value={planarStructure} />
            </div>
            <div class="form-group">
                <label for="continents">Continents</label>
                <div class="array-input">
                    <input type="text" id="continents" placeholder="Add continent" />
                    <button type="button" on:click={() => continents = addToArray(continents, (document.getElementById('continents') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each continents as continent}
                        <span class="array-item">
                            {continent}
                            <button type="button" on:click={() => continents = removeFromArray(continents, continent)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
            <div class="form-group">
                <label for="majorNations">Major Nations</label>
                <div class="array-input">
                    <input type="text" id="majorNations" placeholder="Add nation" />
                    <button type="button" on:click={() => majorNations = addToArray(majorNations, (document.getElementById('majorNations') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each majorNations as nation}
                        <span class="array-item">
                            {nation}
                            <button type="button" on:click={() => majorNations = removeFromArray(majorNations, nation)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
        </div>

        <div class="form-section">
            <h3>World History & Culture</h3>
            <div class="form-group">
                <label for="history">History</label>
                <textarea id="history" bind:value={history} rows="6"></textarea>
            </div>
            <div class="form-group">
                <label for="religions">Religions</label>
                <div class="array-input">
                    <input type="text" id="religions" placeholder="Add religion" />
                    <button type="button" on:click={() => religions = addToArray(religions, (document.getElementById('religions') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each religions as religion}
                        <span class="array-item">
                            {religion}
                            <button type="button" on:click={() => religions = removeFromArray(religions, religion)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
            <div class="form-group">
                <label for="pantheon">Pantheon</label>
                <div class="array-input">
                    <input type="text" id="pantheon" placeholder="Add deity" />
                    <button type="button" on:click={() => pantheon = addToArray(pantheon, (document.getElementById('pantheon') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each pantheon as deity}
                        <span class="array-item">
                            {deity}
                            <button type="button" on:click={() => pantheon = removeFromArray(pantheon, deity)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
        </div>

        <div class="form-section">
            <h3>Additional Information</h3>
            <div class="form-group">
                <label for="notableLandmarks">Notable Landmarks</label>
                <div class="array-input">
                    <input type="text" id="notableLandmarks" placeholder="Add landmark" />
                    <button type="button" on:click={() => notableLandmarks = addToArray(notableLandmarks, (document.getElementById('notableLandmarks') as HTMLInputElement)?.value || '')}>Add</button>
                </div>
                <div class="array-list">
                    {#each notableLandmarks as landmark}
                        <span class="array-item">
                            {landmark}
                            <button type="button" on:click={() => notableLandmarks = removeFromArray(notableLandmarks, landmark)}>×</button>
                        </span>
                    {/each}
                </div>
            </div>
            <div class="form-group">
                <label for="calendarInfo">Calendar Information</label>
                <textarea id="calendarInfo" bind:value={calendarInfo} rows="3"></textarea>
            </div>
            <div class="form-group">
                <label for="establishedMaterial">Established Material</label>
                <textarea id="establishedMaterial" bind:value={establishedMaterial} rows="3"></textarea>
            </div>
        </div>

        <div class="form-actions">
            <button type="submit" class="primary-button">Create World</button>
            <button type="button" class="secondary-button" on:click={resetForm}>Reset Form</button>
        </div>
    </form>
</div> 