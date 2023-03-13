<script lang="ts">
    import type { PageData } from './$types';

    import PostFeed from "$lib/PostFeed.svelte";

    export let data: PageData;

    $: posts = data.posts;
</script>

<PostFeed posts={posts} />
