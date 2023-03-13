<script lang="ts">
    import {goto} from "$app/navigation";

    async function submitForm(e: any) {
        e.preventDefault();

        const name = e.target.name.value;
        const email = e.target.email.value;
        const password = e.target.password.value;
        const confirm = e.target.confirm.value;

        if (password !== confirm) {
            alert("Passwords do not match");
            return;
        }

        const res = await fetch("http://localhost:7100/api/user/create", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                name,
                email,
                password,
            })
        })

        console.log(res);

        if (!res.ok) {
            alert("Something went wrong");
        } else {
            await goto("/login");
        }
    }
</script>

<div>
    <form on:submit={submitForm}>
        <label>
            username
            <input type="text" name="name" />
        </label>
        <label>
            email
            <input type="text" name="email" />
        </label>
        <label>
            password
            <input type="password" name="password" />
        </label>
        <label>
            confirm password
            <input type="password" name="confirm" />
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