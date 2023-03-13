<script lang="ts">
    import {goto} from "$app/navigation";

    async function submitForm(e: any) {
        e.preventDefault();

        const title = e.target.title.value;
        const content = e.target.content.value;

        const res = await fetch("http://localhost:7100/api/post/create", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                title,
                content,
            }),
            credentials: "include",
        });

        if (!res.ok) {
            error = "failed to create post";
            return;
        }

        await goto("/");
    }

    $: error = null;
</script>

<div>
    <form on:submit={submitForm}>
        <label>
            title
            <input type="text" name="title" />
        </label>
        <label>
            content
            <textarea name="content"></textarea>
        </label>
        <button type="submit">Submit</button>
    </form>
</div>

{#if error}
    <span style="color: red">{error}</span>
{/if}

<style>
    div {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100vw;
        height: 100vh;
    }

    form {
        display: flex;
        flex-direction: column;
        width: 300px;
    }

    label {
        display: flex;
        flex-direction: column;
        margin-bottom: 1rem;
    }

    input {
        margin-top: 0.5rem;
        padding: 0.5rem;

        border: 1px solid #ccc;
        border-radius: 4px;
    }

    textarea {
        margin-top: 0.5rem;
        padding: 0.5rem;

        border: 1px solid #ccc;
        border-radius: 4px;

        height: 10rem;
        resize: vertical;
    }

    button {
        padding: 0.5rem;
        border: none;
        border-radius: 4px;
        background-color: #ccc;
        color: #fff;
        cursor: pointer;
    }


    button:hover {
        background-color: #aaa;
    }

</style>