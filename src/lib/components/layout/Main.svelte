<script lang="ts">
    import { activeTab, tabComponents } from '$lib/stores/tabStore';
    import { onMount } from 'svelte';

    // Import all tab components
    import WorldBuilding from './tabs/WorldBuilding.svelte';
    import Campaign from './tabs/Campaign.svelte';
    import Character from './tabs/Character.svelte';
    import Mechanics from './tabs/Mechanics.svelte';
    import AudioVisualAids from './tabs/AudioVisualAids.svelte';
    import Analytics from './tabs/Analytics.svelte';

    // Map component names to actual components
    const components = {
        WorldBuilding,
        Campaign,
        Character,
        Mechanics,
        AudioVisualAids,
        Analytics
    };

    // Get the current component based on active tab
    $: currentComponent = components[tabComponents[$activeTab]];
</script>

<div class="main-container">
    <svelte:component this={currentComponent} />
</div>