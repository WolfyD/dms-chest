/**
 * A generic debounced autocomplete system that can work with any type of data
 */

// Generic type for the suggestion item
export interface SuggestionItem<T> {
    id: number | string;
    label: string;
    data: T;
}

// State interface for the autocomplete system
export interface AutocompleteState<T> {
    suggestions: SuggestionItem<T>[];
    showSuggestions: boolean;
    selectedIndex: number;
    selected?: SuggestionItem<T>;
}

// Configuration options for the autocomplete system
export interface AutocompleteConfig<T> {
    minInputLength?: number;
    debounceTime?: number;
    onSelect?: (item: SuggestionItem<T>) => void;
    transformData?: (data: any) => SuggestionItem<T>[];
}

/**
 * Creates a debounced autocomplete system
 * @param searchFn - Function that performs the search and returns suggestions
 * @param config - Configuration options for the autocomplete system
 * @returns An object with methods to handle autocomplete functionality
 */
export function createDebouncedAutocomplete<T>(
    searchFn: (text: string) => Promise<any[]>,
    config: AutocompleteConfig<T> = {}
) {
    const {
        minInputLength = 3,
        debounceTime = 300,
        onSelect,
        transformData = (data: any[]) => data.map((item, index) => ({
            id: item.id || index,
            label: item.name || item.label || String(item),
            data: item
        }))
    } = config;

    let state: AutocompleteState<T> = {
        suggestions: [],
        showSuggestions: false,
        selectedIndex: -1
    };

    // Debounce utility function
    const debounce = <F extends (...args: any[]) => any>(
        func: F,
        wait: number
    ): ((...args: Parameters<F>) => Promise<ReturnType<F>>) => {
        let timeout: NodeJS.Timeout;
        return function executedFunction(...args: Parameters<F>): Promise<ReturnType<F>> {
            return new Promise((resolve) => {
                clearTimeout(timeout);
                timeout = setTimeout(async () => {
                    try {
                        const result = await func(...args);
                        resolve(result);
                    } catch (error) {
                        console.error('Debounced function error:', error);
                        throw error;
                    }
                }, wait);
            });
        };
    };

    const debouncedSearch = debounce(async (text: string) => {
        console.log('Debounced search called with:', text);
        if (text.length >= minInputLength) {
            console.log('Searching with text:', text);
            const results = await searchFn(text);
            console.log('Search results:', results);
            state.suggestions = transformData(results);
            state.showSuggestions = true;
        } else {
            console.log('Text too short, clearing suggestions');
            state.suggestions = [];
            state.showSuggestions = false;
        }
    }, debounceTime);

    return {
        /**
         * Handles input changes
         * @param text - The input text
         * @returns The current state
         */
        handleInput: async (text: string): Promise<AutocompleteState<T>> => {
            console.log('handleInput called with:', text);
            await debouncedSearch(text);
            console.log('Current state after handleInput:', state);
            return state;
        },

        /**
         * Handles keyboard navigation
         * @param event - The keyboard event
         * @returns The current state
         */
        handleKeydown: (event: KeyboardEvent): AutocompleteState<T> => {
            if (!state.showSuggestions) return state;

            switch (event.key) {
                case 'ArrowDown':
                    event.preventDefault();
                    state.selectedIndex = Math.min(state.selectedIndex + 1, state.suggestions.length - 1);
                    break;
                case 'ArrowUp':
                    event.preventDefault();
                    state.selectedIndex = Math.max(state.selectedIndex - 1, -1);
                    break;
                case 'Enter':
                    event.preventDefault();
                    if (state.selectedIndex >= 0) {
                        const selected = state.suggestions[state.selectedIndex];
                        state.suggestions = [];
                        state.showSuggestions = false;
                        state.selectedIndex = -1;
                        state.selected = selected;
                        onSelect?.(selected);
                        return { ...state, selected };
                    }
                    break;
                case 'Escape':
                    state.showSuggestions = false;
                    state.selectedIndex = -1;
                    break;
            }
            return state;
        },

        /**
         * Handles suggestion selection
         * @param suggestion - The selected suggestion
         * @returns The current state
         */
        selectSuggestion: (suggestion: SuggestionItem<T>): AutocompleteState<T> => {
            state.suggestions = [];
            state.showSuggestions = false;
            state.selectedIndex = -1;
            state.selected = suggestion;
            onSelect?.(suggestion);
            return { ...state, selected: suggestion };
        },

        /**
         * Gets the current state
         * @returns The current state
         */
        getState: (): AutocompleteState<T> => state,

        /**
         * Resets the state
         */
        reset: () => {
            state = {
                suggestions: [],
                showSuggestions: false,
                selectedIndex: -1
            };
        }
    };
} 