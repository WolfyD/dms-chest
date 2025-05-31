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
export function openTool(tool: string) {
    switch (tool) {
        case "world-generator":
            // Create a new window
            const width = 1000;
            const height = 800;
            const left = (window.screen.width - width) / 2;
            const top = (window.screen.height - height) / 2;

            const toolWindow_world_generator = window.open(
                '/world-generator',
                'World Generator',
                `width=${width},height=${height},left=${left},top=${top},resizable=yes,scrollbars=yes,status=no,location=no,addressbar=no,menubar=no,toolbar=no`
            );

            if (toolWindow_world_generator) {
                // Listen for the ready message from the child window
                window.addEventListener('message', function readyHandler(event) {
                    try {
                        const message = JSON.parse(event.data);
                        if (message.type === 'world-generator-ready') {
                            // Remove the listener since we only need it once
                            window.removeEventListener('message', readyHandler);
                            
                            // Now send the data
                            toolWindow_world_generator.postMessage(JSON.stringify({
                                type: "world-generator",
                                data: {
                                    world_id: "1"
                                }
                            }), "*");
                        }
                    } catch (error) {
                        console.error('Error handling message:', error);
                    }
                });

                toolWindow_world_generator.focus();
            }
            break;
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