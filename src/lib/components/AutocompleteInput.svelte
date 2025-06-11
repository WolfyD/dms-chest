<script lang="ts">
    import { createDebouncedAutocomplete, type SuggestionItem } from '$lib/utils/debouncedAutocomplete';
    import { onMount, createEventDispatcher } from 'svelte';
    import { clickOutside } from '$lib/actions/clickOutside';

    // Props
    export let searchFn: (text: string) => Promise<any[]>;
    export let placeholder = 'Type to search...';
    export let minInputLength = 3;
    export let debounceTime = 300;
    export let value: string | number | { id: number; name: string } = '';
    export let onSelect: (item: SuggestionItem<any>) => void = () => {};
    export let class_name: string = '';
    export let icon: string = '';
    export let externalSuggestions: SuggestionItem<any>[] = [];
    export let forceShowSuggestions = false;
    export let required = false;

    // Create event dispatchers
    const dispatch = createEventDispatcher();

    // Local state
    let suggestions: SuggestionItem<any>[] = [];
    let showSuggestions = false;
    let selectedIndex = -1;
    let inputElement: HTMLInputElement;
    let displayValue = '';

    // Watch for external suggestions changes
    $: if (externalSuggestions.length > 0) {
        suggestions = externalSuggestions;
        showSuggestions = true;
    }

    // Watch for force show suggestions
    $: if (forceShowSuggestions) {
        showSuggestions = true;
    }

    // Create autocomplete instance
    const autocomplete = createDebouncedAutocomplete(searchFn, {
        minInputLength,
        debounceTime,
        onSelect: (item) => {
            onSelect(item);
            if (typeof item.data === 'object' && item.data !== null) {
                value = item.data as { id: number; name: string };
            } else {
                // If the id is a number, keep it as a number, otherwise convert to string
                value = typeof item.id === 'number' ? item.id : String(item.id);
            }
            displayValue = item.label;
        }
    });

    // Handle input changes
    async function handleInput(event: Event) {
        const input = event.target as HTMLInputElement;
        displayValue = input.value;
        // Clear selected state when user starts typing
        if (autocomplete.getState().selected) {
            autocomplete.reset();
            value = '';
        }
        const state = await autocomplete.handleInput(displayValue);
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
            if (typeof state.selected.data === 'object' && state.selected.data !== null) {
                value = state.selected.data as { id: number; name: string };
            } else {
                value = typeof state.selected.id === 'number' ? state.selected.id : String(state.selected.id);
            }
            displayValue = state.selected.label;
        }
    }

    // Handle suggestion selection
    function selectSuggestion(suggestion: SuggestionItem<any>) {
        const state = autocomplete.selectSuggestion(suggestion);
        suggestions = state.suggestions;
        showSuggestions = state.showSuggestions;
        selectedIndex = state.selectedIndex;
        onSelect(suggestion);
        if (typeof suggestion.data === 'object' && suggestion.data !== null) {
            value = suggestion.data as { id: number; name: string };
        } else {
            value = typeof suggestion.id === 'number' ? suggestion.id : String(suggestion.id);
        }
        displayValue = suggestion.label;
    }

    // Handle click outside to close suggestions
    function handleClickOutside() {
        showSuggestions = false;
        selectedIndex = -1;
    }

    // Focus the input when the component mounts
    onMount(() => {
        inputElement?.focus();
        // Set initial display value based on value type
        if (typeof value === 'object' && value !== null) {
            displayValue = value.name;
        } else if (typeof value === 'string') {
            displayValue = value;
        } else if (typeof value === 'number') {
            displayValue = String(value);
        }
    });
</script>

<div class="autocomplete-container" use:clickOutside={handleClickOutside}>
    <div class="input-wrapper">
        <input 
            type="text" 
            bind:this={inputElement}
            bind:value={displayValue}
            autocomplete="off"
            {placeholder}
            on:input={handleInput}
            on:keydown={handleKeydown}
            on:focus={() => dispatch('focus')}
            on:click={() => dispatch('click')}
            class={class_name}
            class:required={required}
            required={required}
        />
        {#if icon}
            <i class="ri-{icon}"></i>
        {/if}
    </div>
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
        justify-self: stretch;
    }

    .input-wrapper {
        position: relative;
        display: inline-flexbox;
        align-items: center;
        justify-content: stretch;
        justify-items: stretch;
        justify-self: stretch;
        width: 100%;
    }

    i {
        position: absolute;
        font-size: 1.4rem;
        right: 15%;
        top: 50%;
        transform: translateY(-55%);
        color: var(--color-text-secondary);
        pointer-events: none;
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