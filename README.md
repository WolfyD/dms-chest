# DMS Chest - Tool Development Guide

## Adding a New Tool

### 1. Create the Tool Page
Create a new Svelte page in `src/routes/tools/[tool-name]/+page.svelte`:

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    // Props to receive from parent window
    let props: {
        world_id?: string;
        description?: string;
        // Add other props as needed
    } = {};

    onMount(() => {
        // Initialize your tool here
        console.log('Tool initialized');
    });
</script>

<div class="tool-container">
    <!-- Your tool UI here -->
</div>

<style>
    /* Tool-specific styles */
    .tool-container {
        /* Use variables from variables.css */
        background: var(--color-bg-secondary);
        padding: var(--spacing-md);
    }
</style>
```

### 2. Add Tool Button Handler
In `src/lib/utils/ui.ts`, add your tool to the `openTool` function:

```typescript
export function openTool(tool: string) {
    switch (tool) {
        case "your-tool-name":
            // Navigate to the tool route
            window.location.href = '/tools/your-tool-name';
            break;
        // ... other tools
    }
}
```

### 3. Database Integration
If your tool needs database access:

1. Add necessary tables in `src-tauri/src/database/migrations/migration_001.rs`:
```rust
CREATE TABLE IF NOT EXISTS your_tool_table (
    id INTEGER PRIMARY KEY,
    // Add your columns here
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP
);
```

2. Add query functions in `src-tauri/src/database.rs`:
```rust
#[tauri::command]
pub async fn your_tool_query(state: State<'_, DatabaseState>, params: Vec<String>) -> Result<Vec<serde_json::Value>, String> {
    // Your query implementation
}
```

3. Register the command in `src-tauri/src/main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    // ... other commands
    your_tool_query,
])
```

### 4. Using the Tool
To use your tool from another component:

```typescript
import { openTool } from '$lib/utils/ui';

// Open the tool
openTool('your-tool-name');
```

### 5. Styling Guidelines
1. Use variables from `src/lib/styles/variables.css`
2. Common tool styles are in `src/lib/styles/tool-grid.css`
3. Only create tool-specific styles when necessary
4. Follow the styling hierarchy:
   - Common styles first
   - Tool-specific overrides only when needed

### 6. Best Practices
1. Always handle errors gracefully
2. Use TypeScript for type safety
3. Follow the existing code structure
4. Use the database query pattern for data access
5. Keep tool UI responsive
6. Document any special requirements or dependencies

### 7. Testing
1. Test the tool in isolation
2. Test integration with the main application
3. Test database operations
4. Test error handling
5. Test responsive design

### 8. Example Tool Structure
```
src/
├── routes/
│   └── tools/
│       └── your-tool-name/
│           ├── +page.svelte
│           └── +page.ts
├── lib/
│   └── styles/
│       └── your-tool-name.css (if needed)
└── tauri/
    └── src/
        └── database/
            └── migrations/
                └── your_tool_tables.rs
```

### 9. Important Note
Currently, all tools should be created as paths in the main window rather than as separate windows. This is a temporary solution until we implement proper window management. This means:

1. Tools should be designed to work within the main application layout
2. Use the existing navigation system instead of creating new windows
3. Ensure your tool's UI adapts to the main window's dimensions
4. Follow the main application's styling and layout patterns

This approach ensures consistency and better user experience until we implement a more robust window management system.
