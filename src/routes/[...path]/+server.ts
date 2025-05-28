import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
    // If it's the Chrome DevTools request, return a 200 with empty JSON
    if (params.path === '.well-known/appspecific/com.chrome.devtools.json') {
        return new Response(JSON.stringify({}), {
            headers: {
                'Content-Type': 'application/json'
            }
        });
    }

    // For all other unmatched routes, return 404
    throw error(404, 'Not found');
}; 