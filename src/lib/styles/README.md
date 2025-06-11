# DMS Chest CSS Architecture

## Unified CSS System

The DMS Chest application now uses a unified CSS system that consolidates all styles while preserving the exact current layout and design.

### File Structure

```
src/lib/styles/
├── index.css       # Master CSS file - imports all styles
├── variables.css   # CSS variables and theming system
├── global.css      # Global resets and base styles  
├── layout.css      # Grid-based layout system
├── components.css  # ALL component styles (consolidated)
├── tabs.css        # Tab-specific styles
└── dialogs.css     # Dialog and modal styles
```

**Consolidated into components.css:**
- Tool grid layouts
- Tool cards and buttons  
- Custom dropdown components
- Tab content styles
- Tool box item styles
- Form cards and generators

### Import Order

The CSS files are imported in a specific order to ensure proper cascading:

1. **Variables** - Theme colors, spacing, typography
2. **Global** - Resets and base styles
3. **Layout** - App structure and responsive design
4. **Components** - Reusable UI components
5. **Specific Components** - Specialized component styles

### Usage

To use the unified CSS system, simply import the master file:

```svelte
<script lang="ts">
  import '$lib/styles/index.css';
</script>
```

This single import provides all necessary styles while maintaining proper cascading order.

## Theming System

### CSS Variables

All colors, spacing, and typography are controlled through CSS variables in `variables.css`:

```css
:root {
  /* Theme Colors */
  --color-primary: #24c8db;
  --color-primary-hover: #1ba8b8;
  
  /* Background Colors */
  --color-bg-primary: #f6f6f6;
  --color-bg-secondary: #ffffff;
  
  /* Text Colors */
  --color-text-primary: #0f0f0f;
  --color-text-secondary: #666666;
  
  /* Spacing */
  --spacing-xs: 0.25rem;
  --spacing-sm: 0.5rem;
  --spacing-md: 1rem;
  --spacing-lg: 1.5rem;
  --spacing-xl: 2rem;
}
```

### Creating Custom Themes

To create a custom theme:

1. **Option 1: Override CSS Variables**
   ```css
   .theme-custom {
     --color-primary: #your-color;
     --color-primary-hover: #your-hover-color;
   }
   ```

2. **Option 2: Modify variables.css**
   - Change the root CSS variables directly
   - Uncomment and modify the provided theme examples

3. **Option 3: Dynamic Theming**
   ```javascript
   document.documentElement.style.setProperty('--color-primary', '#new-color');
   ```

### Dark Mode

Dark mode is automatically applied based on system preferences:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --color-bg-primary: #2f2f2f;
    --color-text-primary: #f6f6f6;
    /* ... */
  }
}
```

## Layout Architecture

### Grid-Based Design

The application uses CSS Grid for the main layout:

```css
.app-layout {
  display: grid;
  grid-template-columns: 250px 1fr; /* Sidebar + Main Content */
}
```

### Responsive Breakpoints

- **Desktop (>1024px)**: Sidebar + Main Content
- **Tablet (768px-1024px)**: Stacked layout with horizontal tabs
- **Mobile (<768px)**: Compact stacked layout

### Key Classes

- `.app-layout` - Main grid container
- `.app-sidebar` - Left sidebar with tabs
- `.app-main` - Main content area
- `.tool-grid` - Tool cards container
- `.tabs-container` - Tab navigation

## Best Practices

### Adding New Styles

1. **Use CSS Variables**: Always use existing variables when possible
2. **Component Isolation**: Add component-specific styles to appropriate files
3. **Responsive Design**: Include mobile-first responsive considerations
4. **Consistent Naming**: Follow the existing BEM-like naming convention

### Modifying Existing Styles

1. **Check Variables First**: See if you can achieve your goal by modifying CSS variables
2. **Preserve Layout**: Don't change the core grid structure
3. **Test Responsiveness**: Ensure changes work across all breakpoints
4. **Document Changes**: Update this README for significant modifications

## Maintenance

### CSS File Responsibilities

- **variables.css**: Theme system, colors, spacing, typography
- **layout.css**: App structure, grid system, responsive behavior  
- **components.css**: ALL component styles (tools, dropdowns, forms, cards, etc.)
- **tabs.css**: Tab navigation and related dialogs
- **dialogs.css**: Modal and dialog styles

### Performance Considerations

- All CSS is loaded once via the unified system
- No duplicate styles across files
- Minimal cascade conflicts due to organized import order
- CSS variables enable efficient theming without style duplication 