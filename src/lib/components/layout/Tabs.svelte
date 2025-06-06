<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import AutocompleteInput from '$lib/components/AutocompleteInput.svelte';
    import { getCampaignNames, createCampaign, type CampaignName } from '$lib/utils/campaigns';
    import { onMount } from 'svelte';
    import  CustomDropdown from '$lib/components/CustomDropdown.svelte'; 
    import DropdownGroup from  '$lib/components/CustomDropdown.svelte';
    import { getLocations, checkLocationCount, getFirstNLocations } from '$lib/utils/region';
    import { getCalendars, checkCalendarCount, getCalendarByWorldId } from '$lib/utils/calendars';
    import { getHouseRules, checkHouseRuleCount } from '$lib/utils/houseRules';
    import { getWorlds, checkWorldCount } from '$lib/utils/world';
    import { invoke } from '@tauri-apps/api/core';
    import '$lib/styles/tabs.css';
    import { json } from '@sveltejs/kit';

    function handleTabClick(tabId: string) {
        activeTab.set(tabId);
    }

    let formPage: number = 1;
    let loadedCampaigns: CampaignName[] = [];
    let dropdownOptions: { value: number; label: string }[] = [];
    let campaignId: number = 0;
    let locationObject: { id: number, name: string } = { id: 0, name: '' };
    let calendarObject: { id: number, name: string } = { id: 0, name: '' };
    let houseRulesObject: { id: number, name: string } = { id: 0, name: '' };
    let worldObject: { id: number, name: string } = { id: 0, name: '' };
    let campaignTypeOptions = new Array<DropdownGroup>();
    let campaignType: string = 'other';
    let themes: string[] = [];
    let tones: string[] = [];
    let winConditions: string[] = [];
    let worldSuggestions: { id: number | string, label: string, data: any }[] = [];
    let showWorldSuggestions = false;
    let locationSuggestions: { id: number | string, label: string, data: any }[] = [];
    let showLocationSuggestions = false;
    let calendarSuggestions: { id: number | string, label: string, data: any }[] = [];
    let showCalendarSuggestions = false;
    let houseRuleSuggestions: { id: number | string, label: string, data: any }[] = [];
    let showHouseRuleSuggestions = false;

    onMount(() => {
        getCampaignsFromDatabase();
        setCampaignTypeOptions();
    });

    /**
     * Handles the keydown event
     * @param event
     * @param tabId
     */
    function handleKeydown(event: KeyboardEvent, tabId: string) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            handleTabClick(tabId);
        }
    }

    /**
     * Gets the campaigns from the database
     */
    function getCampaignsFromDatabase() {
        getCampaignNames().then((campaigns) => {
            loadedCampaigns = campaigns;
            dropdownOptions = campaigns.map(campaign => ({
                value: campaign.id,
                label: campaign.name
            }));
        });
    }

    /**
     * Sets the campaign type options
     */
    function setCampaignTypeOptions() {

        campaignTypeOptions = [
            {
                label: "Narrative structure",
                options: [
                    { value: 'linear', label: 'Linear' },
                    { value: 'story-driven', label: 'Story Driven' },
                    { value: 'hybrid', label: 'Hybrid' },
                    { value: 'arc-driven', label: 'Arc Driven' }
                ]
            },
            {
                label: "Gameplay focus",
                options: [
                    { value: 'sandbox', label: 'Sandbox' },
                    { value: 'open-world', label: 'Open World' },
                    { value: 'dungeon-crawl', label: 'Dungeon Crawl' },
                    { value: 'hex-crawl', label: 'Hex Crawl' },
                    { value: 'hack-n-slash', label: 'Hack \'n\' Slash' },
                    { value: 'role-playing-heavy', label: 'Role Playing Heavy' },
                    { value: 'mystery', label: 'Mystery' },
                    { value: 'horror', label: 'Horror' },
                    { value: 'comedy', label: 'Comedy' },
                    { value: 'historical', label: 'Historical' },
                    { value: 'science-fiction', label: 'Science Fiction' },
                    { value: 'epic-fantasy', label: 'Epic Fantasy' },
                    { value: 'dark-fantasy', label: 'Dark Fantasy' }
                ]
            },
            {
                label: "Other",
                options: [
                    { value: 'other', label: "Other" }
                ]
            }
        ];
    }

    /**
     * Handles the selection of a campaign type from the dropdown
     * @param event
     */
    function handleCampaignChange(event: CustomEvent<{ value: string | number; label: string }>) {
        campaignId = Number(event.detail.value);
    }

    /**
     * Opens the new campaign dialog
     */
    function handleCreateNewCampaign() {
        console.log('Create new campaign clicked');
        const dialog = document.getElementById('new-campaign-dialog');
        if (dialog) {
            dialog.showModal();
        }
    }

    /**
     * Handles the submission of the new campaign form
     * @param event
     */
    async function handleCreateNewCampaignSubmit(event: Event) {
        event.preventDefault();
        const formData = new FormData(event.target as HTMLFormElement);
        const newCampaignName = formData.get('campaignName') as string;
        const newCampaignDescription = formData.get('campaignDescription') as string;
        const newCampaignType = campaignType;
        const newPartySize = formData.get('partySize') as string;
        const newPartyLevel = formData.get('partyLevel') as string;
        const newThemes = JSON.stringify(themes);
        const newTones = JSON.stringify(tones);
        const newWinConditions = JSON.stringify(winConditions);
        const newSessionZeroNotes = formData.get('sessionZeroNotes') as string;
        const newPlayerAgreements = formData.get('playerAgreements') as string;
        const newDifficultyLevelString = formData.get('difficultyLevel') as String;
        let newDifficultyLevel = 0;
        switch(newDifficultyLevelString.toLowerCase()) {
            case 'easy':
                newDifficultyLevel = 1;
                break;
            case 'medium':
                newDifficultyLevel = 2;
                break;
            case 'hard':
                newDifficultyLevel = 3;
                break;
            case 'insane':
                newDifficultyLevel = 4;
                break;
            case 'nightmare':
                newDifficultyLevel = 5;
                break;
            case 'custom':
            default:
                newDifficultyLevel = 6;
                break;
        };
        const newStartingLocation = locationObject.id == 0 ? null : locationObject.id;
        const newCalendar = calendarObject.id == 0 ? null : calendarObject.id;
        const newWorldId = worldObject.id;
        const newHouseRules = houseRulesObject.id == 0 ? null : houseRulesObject.id;
        
        console.log("New Campaign name: " + newCampaignName, "New Campaign Description: " + newCampaignDescription, "New Campaign Type: " + newCampaignType, "New Party Size: " + newPartySize, "New Party Level: " + newPartyLevel, "New Themes: " + newThemes, "New Tones: " + newTones, "New Win Conditions: " + newWinConditions, "New Session Zero Notes: " + newSessionZeroNotes, "New Player Agreements: " + newPlayerAgreements, "New Difficulty Level: " + newDifficultyLevel, "New Starting Location: " + newStartingLocation, "New Calendar: " + newCalendar, "New World Id: " + newWorldId, "New House Rules: " + newHouseRules);

        const newCampaignId = await createCampaign(
            /*Campaign name*/           newCampaignName, 
            /*CampaignDescription*/     newCampaignDescription, 
            /*WorldId*/                 newWorldId, 
            /*CampaignType*/            newCampaignType, 
            /*PartySize*/               newPartySize, 
            /*PartyLevel*/              newPartyLevel, 
            /*Themes*/                  newThemes, 
            /*Tones*/                   newTones, 
            /*WinConditions*/           newWinConditions, 
            /*SessionZeroNotes*/        newSessionZeroNotes, 
            /*PlayerAgreements*/        newPlayerAgreements, 
            /*DifficultyLevel*/         newDifficultyLevel, 
            /*StartingLocation*/        newStartingLocation, 
            /*Calendar*/                newCalendar, 
            /*HouseRules*/              newHouseRules
        );

        if(newCampaignId == -1) {
            alert("A campaign with this name already exists!");
            return;
        } else {
            getCampaignsFromDatabase();
            campaignId = newCampaignId;
            closeDialog();
        }

        console.log("newCampaignId", newCampaignId);


        // TODO: Create campaign
    }

    /**
     * Handles the selection of a campaign type
     * @param event
     */
    function handleCampaignTypeChange(event: CustomEvent<{ value: string | number; label: string }>) {
        campaignType = event.detail.value as string;
    }

    /**
     * Closes the new campaign dialog
     * Resets the form
     */
    function closeDialog() {
        const dialog = document.getElementById('new-campaign-dialog');
        if (dialog) {
            dialog.getElementsByTagName('form')[0].reset();
            locationObject = { id: 0, name: '' };
            calendarObject = { id: 0, name: '' };
            houseRulesObject = { id: 0, name: '' };
            themes = [];
            tones = [];
            campaignType = 'other';
            campaignId = 0;
            winConditions = [];
            formPage = 2;
            getPreviousFormPage();
            dialog.close();
        }
    }

    /**
     * Adds a value to an array
     * @param array
     * @param value
     */
    function addToArray(array: string[], value: string) {
        if (value.trim() !== '') {
            array = [...array, value];
            return array;
        }
        return array;
    }

    /**
     * Removes a value from an array
     * @param array
     * @param value
     */
    function removeFromArray(array: string[], value: string) {
        return array.filter(item => item !== value);
    }

    /**
     * Handles the input of a theme
     * @param event
     */
    function handleThemeInput(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault();
            const input = event.target as HTMLInputElement;
            if(input.value.trim() !== '') { 
                themes = [...themes, input.value];
            }
            input.value = '';
        }
    }

    /**
     * Handles the input of a tone
     * @param event
     */
    function handleToneInput(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault();
            const input = event.target as HTMLInputElement;
            if(input.value.trim() !== '') {
                tones = [...tones, input.value];
            }
            input.value = '';
        }
    }

    /**
     * Handles the input of a win condition
     * @param event
     */
    function handleWinConditionInput(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault();
            const input = event.target as HTMLInputElement;
            if(input.value.trim() !== '') {
                winConditions = [...winConditions, input.value];
            }
            input.value = '';
        }
    }

    /**
     * Highlights unfilled elements
     * @param elements
     */
    function highlightUnfilledElements(elements: Element[]) {
        console.log("elements", elements);
        elements.forEach(element => {
            if(element.classList.contains('custom-dropdown-input')) {
                element.parentElement?.firstElementChild?.classList.add('unfilled');
            } else {
                element.classList.add('unfilled');
            }
            window.setTimeout(() => {
                if(element.classList.contains('custom-dropdown-input')) {
                    element.parentElement?.firstElementChild?.classList.remove('unfilled');
                    element.parentElement?.firstElementChild?.classList.add('required');
                } else {
                    element.classList.remove('unfilled');
                }
            }, 2000);
        });
    }

    /**
     * Handles the selection of a next form page
     * and highlights unfilled elements
     */
    function getNextFormPage() {
        let formPages = Array.from(document.querySelectorAll('.campaign-form-container'));
        let currentFormPage = document.getElementById('new-campaign-form-page' + formPage);
        let currentFormElements = Array.from(currentFormPage?.querySelectorAll('input, textarea, select, AutocompleteInput, CustomDropdown') || []);

        let currentUnfilledElements = Array.from(currentFormElements).filter(element => element.value === '' || element.value === 0 || element.value === null);
        let currentUnfilledDropdowns = Array.from(currentFormElements).filter(element => element.classList.contains('custom-dropdown-input'));
        currentUnfilledDropdowns = currentUnfilledDropdowns.filter(element => element.value == '' || element.value == 0 || element.value == null || element.textContent?.startsWith('Select'));
        let newCurrentUnfilledElements: Element[] = [...currentUnfilledElements, ...currentUnfilledDropdowns];

        newCurrentUnfilledElements = newCurrentUnfilledElements.filter(element => element.classList.contains("required"));

        if (newCurrentUnfilledElements.length > 0) {
            const unfilledText = document.querySelector('.unfilled-text');
            unfilledText?.classList.remove('invisible');
            highlightUnfilledElements(newCurrentUnfilledElements);
            return;
        } else {
            const unfilledText = document.querySelector('.unfilled-text');
            unfilledText?.classList.add('invisible');
        }

        formPage++;

        if (formPage > 0) {
            const backButton = document.querySelector('.back-button');
            backButton?.classList.remove('invisible');
            const nextButton = document.querySelector('.next-button');
            nextButton?.classList.add('button-short');
            nextButton?.classList.add('button-right');
        } else {
            const backButton = document.querySelector('.back-button');
            backButton?.classList.add('invisible');
            const nextButton = document.querySelector('.next-button');
            nextButton?.classList.remove('button-short');
            nextButton?.classList.remove('button-right');
        }

        formPages.forEach(page => {
            page.classList.add('invisible');
        });
        formPages[formPage - 1].classList.remove('invisible');
        if (formPage === formPages.length) {
            const createCampaignButton = document.querySelector('.create-campaign-button');
            createCampaignButton?.classList.remove('invisible');
        }
    }

    /**
     * Handles the selection of a previous form page
     */
    function getPreviousFormPage() {
        let formPages = Array.from(document.querySelectorAll('.campaign-form-container'));
        formPage--;
        
        if (formPage <= 1) {
            const backButton = document.querySelector('.back-button');
            backButton?.classList.add('invisible');
            const nextButton = document.querySelector('.next-button');
            nextButton?.classList.remove('button-short');
            nextButton?.classList.remove('button-right');
        }

        if(formPage < formPages.length) {
            const createCampaignButton = document.querySelector('.create-campaign-button');
            createCampaignButton?.classList.add('invisible');
            const nextButton = document.querySelector('.next-button');
            nextButton?.classList.remove('invisible');
        }

        formPages.forEach(page => {
            page.classList.add('invisible');
        });
        formPages[formPage - 1].classList.remove('invisible');
        if (formPage === formPages.length) {
            const createCampaignButton = document.querySelector('.create-campaign-button');
            createCampaignButton?.classList.remove('invisible');
        }
    }

    /**
     * Handles the selection of a location from the autocomplete input
     * @param item
     */
    function handleLocationSelect(item: { id: number | string, label: string, data: any }) {
        locationObject = { id: Number(item.id), name: item.label };
        console.log("locationObject", locationObject);
    }

    /**
     * Handles the selection of a calendar from the autocomplete input
     * @param item
     */
    function handleCalendarSelect(item: { id: number | string, label: string, data: any }) {
        calendarObject = { id: Number(item.id), name: item.label };
        console.log("calendarObject", calendarObject);
    }

    /**
     * Handles the selection of a house rule from the autocomplete input
     * @param item
     */
    function handleHouseRulesSelect(item: { id: number | string, label: string, data: any }) {
        houseRulesObject = { id: Number(item.id), name: item.label };
        console.log("houseRulesObject", houseRulesObject);
    }

    /**
     * Handles the selection of a world from the autocomplete input
     * @param item
     */
    function handleWorldSelect(item: { id: number | string, label: string, data: any }) {
        worldObject = { id: Number(item.id), name: item.label };
        console.log("worldObject", worldObject);
    }

    /**
     * Turns a returned count promise into a number
     * @param promise
     * @returns
     */
    async function countReturnedObject(promise: Promise<{ count: number }>): Promise<number> {
        const result = await promise;
        return result.count;
    }

    /**
     * Loads world suggestions
     */
    async function loadWorldSuggestions() {
        if(worldObject.id == 0 || worldObject.id == null || worldObject.id == undefined) {
            const worlds = await getWorlds("");
            worldSuggestions = worlds.map(world => ({
                id: world.id,
                label: world.name,
                data: world
            }));
            showWorldSuggestions = true;
        }
    }

    /**
     * Loads the first 8 locations from the database
     */
    async function loadLocationSuggestions() {
        if(locationObject.id == 0 || locationObject.id == null || locationObject.id == undefined) {
            const locations = await getFirstNLocations(8);
            locationSuggestions = locations.map(location => ({
                id: location.id,
                label: location.name,
                data: location
            }));
            showLocationSuggestions = true;
        }
    }
    
    /**
     * Loads suggestions for the calendar dropdown from the database
     */
    async function loadCalendarSuggestions() {
        if(calendarObject.id == 0 || calendarObject.id == null || calendarObject.id == undefined) {
            const calendars = await getCalendars("");
            calendarSuggestions = calendars.map(calendar => ({
                id: calendar.id,
                label: calendar.name,
                data: calendar
            }));
            showCalendarSuggestions = true;
        }
    }

    /**
     * Loads suggestions for the house rules dropdown from the database
     */
    async function loadHouseRuleSuggestions() {
        if(houseRulesObject.id == 0 || houseRulesObject.id == null || houseRulesObject.id == undefined) {
            const houseRules = await getHouseRules("");
            houseRuleSuggestions = houseRules.map(houseRule => ({
                id: houseRule.id,
                label: houseRule.name,
                data: houseRule
            }));
            showHouseRuleSuggestions = true;
        }
    }
</script>

<div class="tabs-container">
    <div class="campaign-select-container">
        <CustomDropdown
            options={dropdownOptions}
            value={campaignId}
            required={false}
            placeholder="Select a campaign"
            on:change={handleCampaignChange}
            class_name="campaign-dropdown"
        />
        <button class="new-campaign-button" on:click={() => handleCreateNewCampaign()}>New</button>
    </div>
    {#each Object.entries(categories) as [id, category]}
        <div 
            class="tab {id}" 
            class:selected={$activeTab === id} 
            on:click={() => handleTabClick(id)} 
            on:keydown={(e) => handleKeydown(e, id)} 
            role="tab" 
            tabindex={0}
        >
            <i class={category.icon}></i> 
            <span class="category-name">{category.name}</span>
            <span class="tool-count">{category.tools.length} tools</span>
        </div>
    {/each}
</div>



<!--+ New Campaign Dialog -->
<dialog id="new-campaign-dialog">
    <button class="close-button" on:click={() => closeDialog()}>X</button>
    <form on:submit|preventDefault={handleCreateNewCampaignSubmit}>
        <h2>New Campaign</h2>

        <!--+ Campaign Name, Description, Type, Party Size, Party Level -->
        <div class="campaign-form-container" id="new-campaign-form-page1">
            <input required class="dialog-input required" name="campaignName" autocomplete="off" autofocus={true} type="text" title="Campaign Name" placeholder="Campaign Name" />
            <textarea class="dialog-input" name="campaignDescription" autocomplete="off" title="Campaign Description" placeholder="Campaign Description"></textarea>
            
            <div class="separator"></div>


            <div class="formline-container">
                <CustomDropdown
                    options={campaignTypeOptions}
                    value={campaignId}
                    style="width: 85%; left: 50%; transform: translateX(-50%);"
                    placeholder="Select a campaign type"
                    on:change={handleCampaignTypeChange}
                    class_name="campaign-type-dropdown"
                    title="Campaign Type"
                />
            </div>
            <button class="circle-button add-location-button" title="Add new Location" type="button" on:click={() => { addLocation(); }}>+</button>

            <input required class="dialog-input required" type="number" name="partySize" min="1" max="20" title="Party Size" placeholder="Party Size" />
            <input required class="dialog-input required" type="number" name="partyLevel" min="1" max="20" title="Party Level" placeholder="Party Level" />
        </div>


        <!--+ World, Themes, Tones -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page2">
            <!-- Add theme inserter -->

            <div class="formline-container">
                <AutocompleteInput
                    required={true}
                    class_name="world-autocomplete-container"
                    searchFn={getWorlds}
                    on:click={async () => { 
                        const count = await countReturnedObject(checkWorldCount());
                        if(count < 4) { 
                            await loadWorldSuggestions();
                        }
                    }}
                    bind:value={worldObject}
                    placeholder="Enter world name..."
                    onSelect={handleWorldSelect}
                    icon="earth-line"
                    externalSuggestions={worldSuggestions}
                    forceShowSuggestions={showWorldSuggestions}
                />
                <button class="circle-button add-world-button" title="Add new World" type="button" on:click={() => { addWorld(); }}>+</button>
            </div>

            <div class="separator"></div>

            <div class="form-group">
                <label for="themes">Themes</label>
                <div class="array-container theme-container">
                    <div class="array-input">
                        <input class="add-array-input add-theme-input" type="text" id="new-campaign-themes" placeholder="Add theme" on:keydown={handleThemeInput} />
                        <button class="array-input-button add-theme-button" type="button" on:click={() => { themes = addToArray(themes, (document.getElementById('new-campaign-themes') as HTMLInputElement)?.value || ''); (document.getElementById('new-campaign-themes') as HTMLInputElement).value = '' }}>Add</button>
                    </div>
                    <div class="array-list theme-list">
                        {#each themes as theme}
                            <div class="array-item theme-item">
                                {theme}
                                <button class="array-remove-button remove-theme-button" type="button" on:click={() => themes = removeFromArray(themes, theme)}>×</button>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="separator"></div>

            <div class="form-group">
                <label for="tones">Tones</label>
                <div class="array-container tone-container">
                    <div class="array-input">
                        <input class="add-array-input add-tone-input" type="text" id="new-campaign-tones" placeholder="Add tone" on:keydown={handleToneInput} />
                        <button class="array-input-button add-tone-button" type="button" on:click={() => { tones = addToArray(tones, (document.getElementById('new-campaign-tones') as HTMLInputElement)?.value || ''); (document.getElementById('new-campaign-tones') as HTMLInputElement).value = '' }}>Add</button>
                    </div>
                    <div class="array-list tone-list">
                        {#each tones as tone}
                            <div class="array-item tone-item">
                                {tone}
                                <button class="array-remove-button remove-tone-button" type="button" on:click={() => tones = removeFromArray(tones, tone)}>×</button>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>

        <!--+ Starting Location -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page3">
            <div class="form-group">
                <label for="campaign-setting">Starting Location</label>
                <div class="formline-container">
                    <AutocompleteInput
                        class_name="location-autocomplete-container"
                        searchFn={getLocations}
                        on:click={async () => { 
                            const count = await countReturnedObject(checkLocationCount());
                            await loadLocationSuggestions();
                        }}
                        bind:value={locationObject}
                        placeholder="Enter location name..."
                        onSelect={handleLocationSelect}
                        icon="globe-line"
                        externalSuggestions={locationSuggestions}
                        forceShowSuggestions={showLocationSuggestions}
                    />
                    <button class="circle-button add-location-button" title="Add new Location" type="button" on:click={() => { addLocation(); }}>+</button>
                </div>
            </div>

            <div class="separator"></div>

            <div class="form-group">
                <label for="win-conditions">Win Conditions</label>
                <div class="array-container win-condition-container">
                    <div class="array-input">
                        <input class="add-array-input add-win-condition-input" type="text" id="new-campaign-win-conditions" placeholder="Add win condition" on:keydown={handleWinConditionInput} />
                        <button class="array-input-button add-win-condition-button" type="button" on:click={() => { winConditions = addToArray(winConditions, (document.getElementById('new-campaign-win-conditions') as HTMLInputElement)?.value || ''); (document.getElementById('new-campaign-win-conditions') as HTMLInputElement).value = '' }}>Add</button>
                    </div>

                    <div class="array-list win-condition-list">
                        {#each winConditions as winCondition}
                            <div class="array-item win-condition-item">
                                {winCondition}
                                <button class="array-remove-button remove-win-condition-button" type="button" on:click={() => winConditions = removeFromArray(winConditions, winCondition)}>×</button>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>

        <!--+ Session Zero Notes, Player Agreements -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page4">
            <textarea class="dialog-input" name="sessionZeroNotes" autocomplete="off" title="Session Zero Notes" placeholder="Session Zero Notes"></textarea>
            
            <div class="separator"></div>
            
            <textarea class="dialog-input" name="playerAgreements" autocomplete="off" title="Player Agreements" placeholder="Player Agreements"></textarea>

        </div>

        <!--+ Campaign Goals -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page5">
            <div class="form-group">
                <label for="calendar">Calendar</label>
                <div class="formline-container">
                <AutocompleteInput
                    class_name="calendar-autocomplete-container"
                    searchFn={getCalendars}
                    on:click={async () => { 
                        const count = await countReturnedObject(checkCalendarCount());
                        if(count < 4) { 
                            await loadCalendarSuggestions();
                        }
                    }}
                    bind:value={calendarObject}
                    placeholder="Enter calendar name..."
                    onSelect={handleCalendarSelect}
                    icon="calendar-line"
                    externalSuggestions={calendarSuggestions}
                    forceShowSuggestions={showCalendarSuggestions}
                />
                <button class="circle-button add-calendar-button" title="Add new Calendar" type="button" on:click={() => { addCalendar(); }}>+</button>
                </div>
            </div>

            <div class="separator"></div>

            <div class="form-group">
                <label for="house-rules">House Rules</label>
                <div class="formline-container">
                <AutocompleteInput
                    class_name="house-rules-autocomplete-container"
                    searchFn={getHouseRules}
                    on:click={async () => { 
                        const count = await countReturnedObject(checkHouseRuleCount());
                        if(count < 4) { 
                            await loadHouseRuleSuggestions();
                        }
                    }}
                    bind:value={houseRulesObject}
                    placeholder="Enter house rules..."
                    onSelect={handleHouseRulesSelect}
                    icon="home-gear-line"
                    externalSuggestions={houseRuleSuggestions}
                    forceShowSuggestions={showHouseRuleSuggestions}
                />
                <button class="circle-button add-house-rules-button" title="Add new House Rules" type="button" on:click={() => { addHouseRules(); }}>+</button>
                </div>
            </div>

            <div class="separator"></div>

            <select class="dialog-input" name="difficultyLevel" title="Difficulty Level">
                <option value="easy">Easy</option>
                <option value="medium">Medium</option>
                <option value="hard">Hard</option>
                <option value="insane">Insane</option>
                <option value="nightmare">Nightmare</option>
                <option value="custom">Custom</option>
            </select>
        </div>




        <div class="bottom-container">
            <p class="unfilled-text invisible">Please fill out all required fields</p>
            <div class="button-container">
                <button class="back-button invisible" type="button" on:click={() => getPreviousFormPage()}>Back</button>
                <button class="next-button" type="button" on:click={() => getNextFormPage()}>Next</button>
            </div>
        </div>
        <button class="create-campaign-button button-short button-right invisible" type="submit">Create</button>
    </form>
</dialog>

<style>
    .campaign-dropdown {
        width: 200px;
    }
</style>