import type { LayoutLoad } from './$types';

export const load = (async ({ fetch }) => {
    const response = await fetch("http://localhost:7100/api/user/me", {
        credentials: "include"
    });

    if (!response.ok) {
        return {
            user: null,
            loggedIn: false
        }
    }

    return {
        user: await response.json(),
        loggedIn: true
    }
}) satisfies LayoutLoad;