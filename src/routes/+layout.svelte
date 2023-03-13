<script lang="ts">
    import {setContext} from 'svelte';
    import { writable } from 'svelte/store';

    import type { LayoutData } from './$types';
    import Navbar from "$lib/Navbar.svelte";

    export let data: LayoutData;

    const userStore = writable(null);
    const loggedInStore = writable(false);

    const getUser = async () => {
        const res = await fetch("http://localhost:7100/api/user/me", {
            credentials: "include"
        });

        if (!res.ok) {
            userStore.set(null);
            loggedInStore.set(false);
            return;
        }

        const user = await res.json();
        userStore.set(user);
        loggedInStore.set(true);
    }

    const login = async (email, password) => {
        const res = await fetch("http://localhost:7100/api/user/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                email,
                password
            }),
            credentials: "include"
        });

        await getUser();
    }

    const logout = async () => {
        await fetch("http://localhost:7100/api/user/logout", {
            credentials: "include"
        });

        await getUser();
    }

    $: userStore.set(data.user);
    $: loggedInStore.set(data.loggedIn);

    setContext("auth", {
        user: userStore,
        loggedIn: loggedInStore,
        login,
        logout,
        getUser,
    });

</script>

<svelte:head>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Golos+Text&display=swap" rel="stylesheet">
</svelte:head>

<style>
    :global(body) {
        font-family: 'Golos Text', sans-serif;
        margin: 0;
    }
</style>

<Navbar />
<slot />