<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { fade, fly } from 'svelte/transition';
    import { clickOutside } from '$lib/actions/clickOutside';
    import '$lib/styles/customDropdown.css';

    type DropdownOption = {
        value: string | number;
        label: string;
    };

    type DropdownGroup = {
        label: string;
        options: DropdownOption[];
    };

    export let options: (DropdownOption | DropdownGroup)[] = [];
    export let value: string | number = '';
    export let placeholder: string = 'Select an option';
    export let disabled: boolean = false;
    export let name: string = '';
    export let id: string = '';
    export let class_name: string = '';
    export let required: boolean = false;
    export let style: string = 'width: 100%;';
    export let title: string = '';


    let isOpen = false;
    let selectedLabel = '';
    let dropdownRef: HTMLElement;

    const dispatch = createEventDispatcher<{
        change: { value: string | number; label: string };
    }>();

    function findOptionInGroups(value: string | number): DropdownOption | undefined {
        for (const item of options) {
            if ('options' in item) {
                // It's a group
                const found = item.options.find(opt => opt.value === value);
                if (found) return found;
            } else {
                // It's a single option
                if (item.value === value) return item;
            }
        }
        return undefined;
    }

    $: {
        const selected = findOptionInGroups(value);
        selectedLabel = selected ? selected.label : '';
    }

    function handleSelect(option: DropdownOption) {
        value = option.value;
        selectedLabel = option.label;
        isOpen = false;
        dispatch('change', option);
    }

    function toggleDropdown() {
        if (!disabled) {
            isOpen = !isOpen;
        }
    }

    function findNextOption(currentValue: string | number): DropdownOption | undefined {
        let found = false;
        for (const item of options) {
            if ('options' in item) {
                // It's a group
                for (const opt of item.options) {
                    if (found) return opt;
                    if (opt.value === currentValue) found = true;
                }
            } else {
                // It's a single option
                if (found) return item;
                if (item.value === currentValue) found = true;
            }
        }
        // If we haven't found the next option, return the first one
        const firstItem = options[0];
        if ('options' in firstItem) {
            return firstItem.options[0];
        }
        return firstItem as DropdownOption;
    }

    function findPreviousOption(currentValue: string | number): DropdownOption | undefined {
        let previous: DropdownOption | undefined;
        for (const item of options) {
            if ('options' in item) {
                // It's a group
                for (const opt of item.options) {
                    if (opt.value === currentValue) return previous;
                    previous = opt;
                }
            } else {
                // It's a single option
                if (item.value === currentValue) return previous;
                previous = item as DropdownOption;
            }
        }
        // If we haven't found the previous option, return the last one
        const lastItem = options[options.length - 1];
        if ('options' in lastItem) {
            return lastItem.options[lastItem.options.length - 1];
        }
        return lastItem as DropdownOption;
    }

    function handleKeydown(event: KeyboardEvent) {
        if (disabled) return;

        switch (event.key) {
            case 'Enter':
            case ' ':
                event.preventDefault();
                toggleDropdown();
                break;
            case 'Escape':
                isOpen = false;
                break;
            case 'ArrowDown':
                event.preventDefault();
                if (!isOpen) {
                    isOpen = true;
                } else {
                    const nextOption = findNextOption(value);
                    if (nextOption) handleSelect(nextOption);
                }
                break;
            case 'ArrowUp':
                event.preventDefault();
                if (!isOpen) {
                    isOpen = true;
                } else {
                    const prevOption = findPreviousOption(value);
                    if (prevOption) handleSelect(prevOption);
                }
                break;
        }
    }
    

    export { type DropdownOption, type DropdownGroup }; 
</script>

<div 
    class="custom-dropdown {class_name}" 
    class:disabled 
    class:open={isOpen}
    bind:this={dropdownRef}
    use:clickOutside={() => isOpen = false}
    on:keydown={handleKeydown}
    style={style}
    role="combobox"
    aria-expanded={isOpen}
    aria-haspopup="listbox"
    aria-controls="dropdown-list"
    tabindex="0"
    title={title}
>
    <div class="dropdown-header" on:click={toggleDropdown}>
        <span class="selected-value">{selectedLabel || placeholder}</span>
        <span class="dropdown-arrow">▼</span>
    </div>

    {#if isOpen}
        <div 
            class="dropdown-list" 
            id="dropdown-list" 
            role="listbox"
            transition:fly={{ y: -10, duration: 200 }}
        >
            {#each options as item}
                {#if 'options' in item}
                    <div class="dropdown-group">
                        <div class="group-label">{item.label}</div>
                        {#each item.options as option}
                            <menuitem
                                class="dropdown-item"
                                class:selected={option.value === value}
                                on:click={() => handleSelect(option)}
                                role="option"
                                aria-selected={option.value === value}
                            >
                                {option.label}
                            </menuitem>
                        {/each}
                    </div>
                {:else}
                    <menuitem
                        class="dropdown-item"
                        class:selected={item.value === value}
                        on:click={() => handleSelect(item)}
                        role="option"
                        aria-selected={item.value === value}
                    >
                        {item.label}
                    </menuitem>
                {/if}
            {/each}
        </div>
    {/if}

    <input 
        type="hidden" 
        class="custom-dropdown-input"
        class:required={required ? 'required' : 'not-required'}
        {name} 
        {id} 
        {value} 
        {disabled}
    />
</div>