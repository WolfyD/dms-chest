// Centralized dialog utilities for the DMS Chest application

/**
 * Opens a modal dialog by its ID
 * @param dialogId - The ID of the dialog to open
 */
export function openDialog(dialogId: string): void {
    const dialog = document.getElementById(dialogId) as HTMLDialogElement;
    if (dialog) {
        dialog.showModal();
    }
}

/**
 * Closes a modal dialog by its ID
 * @param dialogId - The ID of the dialog to close
 */
export function closeDialog(dialogId: string): void {
    const dialog = document.getElementById(dialogId) as HTMLDialogElement;
    if (dialog) {
        dialog.close();
    }
}

/**
 * Resets a form within a dialog
 * @param dialogId - The ID of the dialog containing the form
 */
export function resetDialogForm(dialogId: string): void {
    const dialog = document.getElementById(dialogId) as HTMLDialogElement;
    if (dialog) {
        const form = dialog.querySelector('form');
        if (form) {
            form.reset();
        }
    }
}

/**
 * Shows a create object dialog based on object type
 * @param objectType - The type of object to create (Location, World, Calendar, etc.)
 */
export function showCreateObjectDialog(objectType: string): void {
    // This function would need to be implemented based on the specific dialog system
    console.log(`Opening create dialog for ${objectType}`);
    // For now, we'll just log the action - implementation would depend on the dialog structure
}

// Helper functions for common object creation dialogs
export function addLocation(): void {
    showCreateObjectDialog('Location');
}

export function addWorld(): void {
    showCreateObjectDialog('World');
}

export function addCalendar(): void {
    showCreateObjectDialog('Calendar');
}

export function addHouseRules(): void {
    showCreateObjectDialog('House Rules');
}

export function addMap(): void {
    showCreateObjectDialog('Map');
} 