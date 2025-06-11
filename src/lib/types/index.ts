// Centralized type definitions for the DMS Chest application

// Dropdown component types
export interface DropdownOption {
    value: string | number;
    label: string;
    id?: string | number;
}

export interface DropdownGroup {
    label: string;
    options: DropdownOption[];
}

// Database entity types
export interface CampaignName {
    name: string;
    id: number;
}

export interface LocationEntity {
    id: number;
    name: string;
    type?: string;
    parent_id?: number;
    has_parent?: boolean;
    has_children?: boolean;
}

export interface WorldEntity {
    id: number;
    name: string;
}

export interface CalendarEntity {
    id: number;
    name: string;
}

export interface MapEntity {
    id: number;
    name: string;
}

export interface HouseRulesEntity {
    id: number;
    name: string;
}

// Autocomplete types
export interface SuggestionItem<T = any> {
    id: number | string;
    label: string;
    data: T;
}

// Form validation types
export interface ValidationResult {
    isValid: boolean;
    errors: string[];
}

// Database query types
export interface DatabaseQueryResult<T = any> {
    count?: number;
    data?: T[];
} 