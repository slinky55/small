<script lang="ts">

    import {getContext} from "svelte";

    const { user, loggedIn, logout } = getContext("auth");
</script>

<div>
    {#if $loggedIn}
        <a href="/dashboard">Dashboard</a>
        <a href="/" on:click={logout}>Logout</a>
    {:else}
        <a href="/login">Login</a>
        <a href="/signup">Sign Up</a>
    {/if}
</div>

<style>
    div {
        border-bottom: 1px solid #ccc;
        height: 10vh;
    }
</style>