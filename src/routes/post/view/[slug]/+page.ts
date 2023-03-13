import type { PageLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
    const { slug } = params;
    const res = await fetch("http://localhost:7100/api/post/get/" + slug);

    if (!res.ok) {
        return {
            post: null,
        }
    }

    const post = await res.json();
    console.log(post);

    return {
        post: post
    }
}) satisfies PageLoad;