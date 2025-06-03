<script lang="ts">
    import { createDebouncedAutocomplete, type SuggestionItem } from '$lib/utils/debouncedAutocomplete';
    import { onMount } from 'svelte';

    // Props
    export let searchFn: (text: string) => Promise<any[]>;
    export let placeholder = 'Type to search...';
    export let minInputLength = 3;
    export let debounceTime = 300;
    export let value = '';
    export let onSelect: (item: SuggestionItem<any>) => void = () => {};

    // Local state
    let suggestions: SuggestionItem<any>[] = [];
    let showSuggestions = false;
    let selectedIndex = -1;
    let inputElement: HTMLInputElement;

    // Create autocomplete instance
    const autocomplete = createDebouncedAutocomplete(searchFn, {
        minInputLength,
        debounceTime,
        onSelect: (item) => {
            onSelect(item);
        }
    });

    // Handle input changes
    async function handleInput(event: Event) {
        const input = event.target as HTMLInputElement;
        value = input.value;
        // Clear selected state when user starts typing
        if (autocomplete.getState().selected) {
            autocomplete.reset();
        }
        const state = await autocomplete.handleInput(value);
        suggestions = state.suggestions;
        showSuggestions = state.showSuggestions;
        selectedIndex = state.selectedIndex;
    }

    // Handle keyboard navigation
    function handleKeydown(event: KeyboardEvent) {
        // Don't handle selection if user is typing
        if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp' && event.key !== 'Enter' && event.key !== 'Escape') {
            return;
        }
        const state = autocomplete.handleKeydown(event);
        suggestions = state.suggestions;
        showSuggestions = state.showSuggestions;
        selectedIndex = state.selectedIndex;
        if (state.selected) {
            onSelect(state.selected);
        }
    }

    // Handle suggestion selection
    function selectSuggestion(suggestion: SuggestionItem<any>) {
        const state = autocomplete.selectSuggestion(suggestion);
        suggestions = state.suggestions;
        showSuggestions = state.showSuggestions;
        selectedIndex = state.selectedIndex;
        onSelect(suggestion);
    }

    // Focus the input when the component mounts
    onMount(() => {
        inputElement?.focus();
    });
</script>

<div class="autocomplete-container">
    <input 
        type="text" 
        bind:this={inputElement}
        {value}
        autocomplete="off"
        {placeholder}
        on:input={handleInput}
        on:keydown={handleKeydown}
    />
    {#if showSuggestions && suggestions.length > 0}
        <ul class="suggestions-list">
            {#each suggestions as suggestion, i}
                <li>
                    <menuitem
                        class="suggestion-item"
                        class:selected={i === selectedIndex}
                        on:click={() => selectSuggestion(suggestion)}
                        on:mouseover={() => selectedIndex = i}
                        on:focus={() => selectedIndex = i}
                    >
                        {suggestion.label}
                    </menuitem>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style>
    .autocomplete-container {
        position: relative;
        width: 100%;
    }

    input {
        width: 100%;
        padding: var(--spacing-sm);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-sm);
        background: var(--color-bg-secondary);
        color: var(--color-text-primary);
    }

    .suggestions-list {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        margin: 0;
        padding: 0;
        list-style: none;
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border);
        border-radius: var(--radius-sm);
        max-height: 200px;
        overflow-y: auto;
        z-index: 1000;
    }

    .suggestion-item {
        display: block;
        padding: var(--spacing-sm);
        cursor: pointer;
        width: auto;
    }

    .suggestion-item:hover,
    .suggestion-item.selected {
        background: var(--color-primary);
        color: var(--color-bg-primary);
    }
</style> 