<script lang="ts">
    import {getContext} from "svelte";

    const { user, loggedIn } = getContext("auth");

    console.log(user);
</script>

{#if $loggedIn}
    <h1>Dashboard</h1>
    <p>Welcome, {$user.name}!</p>
{:else}
    <h1>Not logged in</h1>
    <p>You need to be logged in to see this page.</p>
{/if}