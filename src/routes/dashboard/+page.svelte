<script lang="ts">
    import {getContext} from "svelte";
    import type { PageData } from './$types';
    import PostFeed from "$lib/PostFeed.svelte";

    export let data: PageData;

    $: posts = data.posts;

    const { user, loggedIn } = getContext("auth");
</script>

{#if $loggedIn}
    <PostFeed posts={posts} />
{:else}
    <h1>Welcome Guest</h1>
{/if}