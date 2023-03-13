import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    const res = await fetch("http://localhost:7100/api/user/posts", {
        credentials: "include",
    });

    if (!res.ok) {
        return {
            posts: [],
        }
    }

    return {
        posts: await res.json(),
    }
}) satisfies PageLoad;