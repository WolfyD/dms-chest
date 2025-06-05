<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import AutocompleteInput from '$lib/components/AutocompleteInput.svelte';
    import { getCampaignNames, type CampaignName } from '$lib/utils/campaigns';
    import { onMount } from 'svelte';
    import  CustomDropdown from '$lib/components/CustomDropdown.svelte'; 
    import DropdownGroup from  '$lib/components/CustomDropdown.svelte';
    import { getLocations } from '$lib/utils/region';
    import { getCalendars } from '$lib/utils/calendars';
    import { invoke } from '@tauri-apps/api/core';
    import '$lib/styles/tabs.css';

    function handleTabClick(tabId: string) {
        activeTab.set(tabId);
    }

    let formPage: number = 1;
    let loadedCampaigns: CampaignName[] = [];
    let dropdownOptions: { value: number; label: string }[] = [];
    let locationDropdownOptions: { value: number; label: string }[] = [];
    let locationSuggestions: { name: string, id: number }[] = [];
    let calendarSuggestions: { name: string, id: number }[] = [];
    let campaignId: number = 0;
    let locationObject: { id: number, name: string } = { id: 0, name: '' };
    let calendarObject: { id: number, name: string } = { id: 0, name: '' };
    let campaignTypeOptions = new Array<DropdownGroup>();
    let campaignType: string = 'other';
    let themes: string[] = [];
    let tones: string[] = [];
    let winConditions: string[] = [];

    onMount(() => {
        getCampaignsFromDatabase();
        setCampaignTypeOptions();
    });

    function handleKeydown(event: KeyboardEvent, tabId: string) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            handleTabClick(tabId);
        }
    }

    function getCampaignsFromDatabase() {
        getCampaignNames().then((campaigns) => {
            loadedCampaigns = campaigns;
            dropdownOptions = campaigns.map(campaign => ({
                value: campaign.id,
                label: campaign.name
            }));
        });
    }

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
    function handleCampaignChange(event: CustomEvent<{ value: string | number; label: string }>) {
        campaignId = Number(event.detail.value);
    }

    function handleCreateNewCampaign() {
        console.log('Create new campaign clicked');
        const dialog = document.getElementById('new-campaign-dialog');
        if (dialog) {
            dialog.showModal();
        }
    }

    function handleCreateNewCampaignSubmit(event: Event) {
        event.preventDefault();
        const formData = new FormData(event.target as HTMLFormElement);
        const campaignName = formData.get('campaignName') as string;
        const campaignDescription = formData.get('campaignDescription') as string;
        const newCampaignType = campaignType;
        console.log(campaignName, campaignDescription, newCampaignType);
    }

    function handleCampaignTypeChange(event: CustomEvent<{ value: string | number; label: string }>) {
        campaignType = event.detail.value as string;
    }

    function closeDialog() {
        const dialog = document.getElementById('new-campaign-dialog');
        if (dialog) {
            dialog.close();
        }
    }

    function addToArray(array: string[], value: string) {
        if (value.trim() !== '') {
            array = [...array, value];
            return array;
        }
        return array;
    }

    function removeFromArray(array: string[], value: string) {
        return array.filter(item => item !== value);
    }

    function handleThemeInput(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            const input = event.target as HTMLInputElement;
            themes = [...themes, input.value];
            input.value = '';
        }
    }

    function handleToneInput(event: KeyboardEvent) {

        if (event.key === 'Enter') {
            const input = event.target as HTMLInputElement;
            tones = [...tones, input.value];
            input.value = '';
        }
    }

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
                } else {
                    element.classList.remove('unfilled');
                }
            }, 2000);
        });
    }

    function getNextFormPage() {
        let formPages = Array.from(document.querySelectorAll('.campaign-form-container'));
        let currentFormPage = document.getElementById('new-campaign-form-page' + formPage);
        let currentFormElements = Array.from(currentFormPage?.querySelectorAll('input, textarea, select, AutocompleteInput, CustomDropdown') || []);

        let currentUnfilledElements = Array.from(currentFormElements).filter(element => element.value === '' || element.value === 0 || element.value === null);
        let currentUnfilledDropdowns = Array.from(currentFormElements).filter(element => element.classList.contains('custom-dropdown-input'));
        currentUnfilledDropdowns = currentUnfilledDropdowns.filter(element => element.value == '' || element.value == 0 || element.value == null || element.textContent?.startsWith('Select'));
        let newCurrentUnfilledElements: Element[] = [...currentUnfilledElements, ...currentUnfilledDropdowns];
        console.log("currentUnfilledElements", newCurrentUnfilledElements);

        newCurrentUnfilledElements = newCurrentUnfilledElements.filter(element => element.classList.contains("required"));

        if (newCurrentUnfilledElements.length > 0) {
            console.log("highlighting unfilled elements", newCurrentUnfilledElements);
            highlightUnfilledElements(newCurrentUnfilledElements);
            //return;
        }

        //onsole.log("currentUnfilledElements", currentUnfilledElements);





        formPage++;

        console.log("formPages",formPages);
        formPages.forEach(page => {
            console.log("page",page);
            page.classList.add('invisible');
        });
        formPages[formPage - 1].classList.remove('invisible');
        if (formPage === formPages.length) {
            const createCampaignButton = document.querySelector('.create-campaign-button');
            createCampaignButton?.classList.remove('invisible');
        }
    }

    function handleLocationSelect(item: { id: number | string, label: string, data: any }) {
        locationObject = { id: Number(item.id), name: item.label };
        console.log("locationObject", locationObject);
    }

    function handleCalendarSelect(item: { id: number | string, label: string, data: any }) {
        calendarObject = { id: Number(item.id), name: item.label };
        console.log("calendarObject", calendarObject);
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
    <form on:submit={handleCreateNewCampaignSubmit}>
        <h2>New Campaign</h2>

        <!--+ Campaign Name, Description, Type, Party Size, Party Level -->
        <div class="campaign-form-container" id="new-campaign-form-page1">
            <input required class="dialog-input required" name="campaignName" autocomplete="off" autofocus={true} type="text" title="Campaign Name" placeholder="Campaign Name" />
            <textarea required class="dialog-input required" name="campaignDescription" autocomplete="off" title="Campaign Description" placeholder="Campaign Description"></textarea>
            
            <div class="separator"></div>

            <div class="formline-container">
                <CustomDropdown
                    options={campaignTypeOptions}
                    value={campaignId}
                    style="width: 85%; left: 50%; transform: translateX(-50%);"
                    placeholder="Select a campaign"
                    on:change={handleCampaignTypeChange}
                    class_name="campaign-type-dropdown"
                    title="Campaign Type"
                />
            </div>

            <input required class="dialog-input required" type="number" name="partySize" min="1" max="20" title="Party Size" placeholder="Party Size" />
            <input required class="dialog-input required" type="number" name="partyLevel" min="1" max="20" title="Party Level" placeholder="Party Level" />
        </div>


        <!--+ Themes, Tones -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page2">
            <!-- Add theme inserter -->
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
                <AutocompleteInput
                    class_name="location-autocomplete-container"
                    searchFn={getLocations}
                    bind:value={locationObject}
                    placeholder="Enter location name..."
                    onSelect={handleLocationSelect}
                />
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
            <textarea class="dialog-input required" name="sessionZeroNotes" autocomplete="off" title="Session Zero Notes" placeholder="Session Zero Notes"></textarea>
            
            <div class="separator"></div>
            
            <textarea class="dialog-input required" name="playerAgreements" autocomplete="off" title="Player Agreements" placeholder="Player Agreements"></textarea>

        </div>

        <!--+ Campaign Goals -->
        <div class="campaign-form-container invisible" id="new-campaign-form-page5">
            <div class="form-group">
                <label for="calendar">Calendar</label>
                <AutocompleteInput
                    class_name="calendar-autocomplete-container"
                    searchFn={getCalendars}
                    bind:value={calendarObject}
                    placeholder="Enter calendar name..."
                    onSelect={handleCalendarSelect}
                />
            </div>
        </div>




        <button class="next-button" type="button" on:click={() => getNextFormPage()}>Next</button>
        <button class="create-campaign-button invisible" type="submit">Create</button>
    </form>
</dialog>

<style>
    .campaign-dropdown {
        width: 200px;
    }
</style>