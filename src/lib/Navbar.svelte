<script lang="ts">

    import {getContext} from "svelte";
    import {goto} from "$app/navigation";

    const { user, loggedIn, logout } = getContext("auth");
</script>

<nav>
    <div class="logo">
        <a href="/"><h1>SM</h1></a>
    </div>

    <div class="menu">
        {#if $loggedIn}
            <a href="/create">Create Post</a>
            <a href="/dashboard">Dashboard</a>
        {:else}
            <a href="/login">Login</a>
            <a href="/signup">Sign Up</a>
        {/if}
    </div>
</nav>

<style>
    nav {
        display: flex;
        justify-content: center;
        align-items: center;
        column-gap: 50rem;

        height: 10vh;

        border-bottom: 1px solid #ccc;
    }

    .logo {
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 18px;

        background-color: black;
        color: white;

        width: 60px;
        height: 40px;
    }

    .menu {
        display: flex;
        align-items: center;
        justify-content: center;
        column-gap: 2rem;
    }

    a {
        text-decoration: none;
        color: black;
    }

    .logo a {
        color: white;
    }
</style>