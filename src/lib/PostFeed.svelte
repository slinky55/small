<script lang="ts">
    import {goto} from "$app/navigation";

    export let posts: { id: number, text: string, user_id: number }[];

    const viewPost = async (id: number) => {
        await goto(`/post/view/${id}`);
    }
</script>

<div class="container">
    {#each posts as post}
        <div
                class="postDisplay"
                on:click={viewPost(post.id)}
                on:keydown
        >
            <div>
                <h1>{post.title}</h1>
                <p>@{post.user_id}</p>
            </div>
        </div>
    {/each}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;

        row-gap: 5vh;

        width: 100vw;

        padding-top: 3rem;
    }

    .postDisplay {
        display: flex;
        flex-direction: row;
        align-items: center;

        padding-left: 1rem;

        border: 1px solid #ccc;

        width: 55vw;
        height: 15vh;

        cursor: pointer;
    }
</style>
