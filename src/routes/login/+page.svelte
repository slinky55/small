<script lang="ts">
    import {goto} from "$app/navigation";
    import {getContext} from "svelte";

    const { login } = getContext("auth");

    async function submitForm(e: any) {
        e.preventDefault();

        const email = e.target.email.value;
        const password = e.target.password.value;

        await login(email, password);

        await goto("/dashboard");
    }
</script>

<div>
    <form on:submit={submitForm}>
        <label>
            email
            <input type="email" name="email" />
        </label>
        <label>
            password
            <input type="password" name="password" />
        </label>
        <button type="submit">Submit</button>
    </form>
</div>

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