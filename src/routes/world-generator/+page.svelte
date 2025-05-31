<script lang="ts">
    import { onMount } from 'svelte';
    import WorldGenerator from '$lib/components/tools/world-generator/WorldGenerator.svelte';

    let worldId: string | null = null;

    onMount(() => {
        // Listen for messages from the parent window
        window.addEventListener('message', (event) => {
            try {
                const message = JSON.parse(event.data);
                if (message.type === 'world-generator') {
                    worldId = message.data.world_id;
                    console.log('Received world ID:', worldId);
                }
            } catch (error) {
                console.error('Error parsing message:', error);
            }
        });

        // Notify parent window that we're ready
        window.opener?.postMessage(JSON.stringify({
            type: 'world-generator-ready'
        }), '*');
    });
</script>

<div class="tool-window">
    <WorldGenerator {worldId} />
</div>

<style>
    .tool-window {
        padding: var(--spacing-lg);
        height: 100vh;
        overflow-y: auto;
        background: var(--color-bg-primary);
    }
</style> 