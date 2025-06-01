import { goto } from '$app/navigation';
import { toolData } from '$lib/stores/toolData';
// This file will contain all the UI utilities for the app

// This function checks which page is loaded and adds the class "selected" to the correct tab
export function checkPage() {
    const path = window.location.pathname;
    const tab = document.querySelector(`.tab[href="${path}"]`);
    if (tab) {
        tab.classList.add("selected");
    }
}

/**
 * Adjusts the height of the app layout to fit within the available viewport height
 * without causing parent scrolling. This should be called on window resize.
 */
export function adjustAppLayoutHeight(): void {
    const appLayout = document.querySelector('.app-layout');
    const header = document.querySelector('.app-header-title');
    const footer = document.querySelector('.app-footer');
    const appHeader = document.querySelector('.app-header');
    const appContent = document.querySelector('.app-content');
    if (!appLayout || !header || !footer || !appHeader || !appContent) return;
    
    const headerHeight = header.getBoundingClientRect().height;
    const footerHeight = footer.getBoundingClientRect().height;
    // Get the viewport height
    const viewportHeight = window.innerHeight;
    
    // Get the position of the app layout relative to the viewport
    const rect = appLayout.getBoundingClientRect();
    
    // Calculate available height (viewport height minus top position)
    const availableHeight = viewportHeight - rect.top;

    const height = availableHeight - footerHeight - 10;
    
    // Set the max height of the app layout
    (appLayout as HTMLElement).style.maxHeight = `${height}px`;
    (appLayout as HTMLElement).style.minHeight = `${height}px`;
    (appContent as HTMLElement).style.maxHeight = `${height}px`;
    (appHeader as HTMLElement).style.maxHeight = `${height}px`;

    console.log("Available height: ", availableHeight, "Header height: ", headerHeight, "Footer height: ", footerHeight);
}

/**
 * Initializes the height adjustment functionality
 * This should be called when the app starts
 * @returns A cleanup function to remove the event listener
 */
export function initHeightAdjustment(): () => void {
    // Initial adjustment
    adjustAppLayoutHeight();
    
    // Add resize listener
    window.addEventListener('resize', adjustAppLayoutHeight);
    
    // Return cleanup function
    return () => {
        window.removeEventListener('resize', adjustAppLayoutHeight);
    };
}

/**
 * Opens a tool
 * @param tool - The tool to open
 */
export function openTool(tool: string, tool_description: string, id: string) {
    toolData.set({
        id: id,
        description: tool_description
    });

    switch (tool) {
        case "world-generator": goto('/world-generator'); break;
        case "plot-generator":
        case "prophecy-creator":
        case "region-generator":
        case "culture-tools":
        case "rumor-builder":
        case "timeline-and-history":
        case "language-tools":
        case "astronomical-tools":
        case "log-taker":
        case "quest-tracker":
        case "campaign-manager":
        case "character-creator":
        case "class-creator":
        case "monster-creator":
        case "item-creator":
        case "spell-creator":
        case "map-creator":
        case "image-creator":
            console.log(`Tool ${tool} not implemented yet`);
            break;
    }

    console.log("Opening tool: ", tool);
}