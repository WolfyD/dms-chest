<script lang="ts">
    import { activeTab, categories } from '$lib/stores/tabStore';
    import '$lib/styles/tabs.css';
    import { getCampaignNames, type CampaignName } from '$lib/utils/campaigns';
    import { onMount } from 'svelte';
    import  CustomDropdown from '$lib/components/CustomDropdown.svelte'; 
    import DropdownOption from '$lib/components/CustomDropdown.svelte';
    import DropdownGroup from  '$lib/components/CustomDropdown.svelte';
    import { invoke } from '@tauri-apps/api/core';

    function handleTabClick(tabId: string) {
        activeTab.set(tabId);
    }

    let loadedCampaigns: CampaignName[] = [];
    let campaignId: number = 0;
    let dropdownOptions: { value: number; label: string }[] = [];
    let campaignTypeOptions = new Array<DropdownGroup>();
    let campaignType: string = 'other';
    let themes: string[] = [];

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

    function handleThemeInput(event: Event) {
        const input = event.target as HTMLInputElement;
        themes.push(input.value);
        input.value = '';
    }
</script>

<div class="tabs-container">
    <div class="campaign-select-container">
        <CustomDropdown
            options={dropdownOptions}
            value={campaignId}
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

<dialog id="new-campaign-dialog">
    <button class="close-button" on:click={() => closeDialog()}>X</button>
    <form on:submit={handleCreateNewCampaignSubmit}>
        <h2>New Campaign</h2>
        <input required class="dialog-input" name="campaignName" type="text" placeholder="Campaign Name" />
        <textarea required class="dialog-input" name="campaignDescription" placeholder="Campaign Description"></textarea>
        <CustomDropdown
            options={campaignTypeOptions}
            value={campaignId}
            placeholder="Select a campaign"
            on:change={handleCampaignTypeChange}
            class_name="campaign-dropdown"
        />
        <input required type="number" name="partySize" min="1" max="20" placeholder="Party Size" />
        <input required type="number" name="partyLevel" min="1" max="20" placeholder="Party Level" />

        <!-- Add theme inserter -->
        <input type="text" name="theme" placeholder="Theme" on:input={handleThemeInput} />
        {#each themes as theme}
            <div class="theme-container">{theme} <button class="remove-theme-button">X</button></div>
        {/each}
        <button type="submit">Create</button>
    </form>
</dialog>

<style>
    .campaign-dropdown {
        width: 200px;
    }
</style>