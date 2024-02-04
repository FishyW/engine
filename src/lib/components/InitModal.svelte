<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    type FormEvent = SubmitEvent & {currentTarget: HTMLFormElement};

    function validateForm(event: FormEvent) {
        const form = event.currentTarget;
        const data = new FormData(event.currentTarget);
        const path = data.get("path");

        // path is empty
        if (!path) {
            alert("Select a project folder!");
            return;
        }
        dispatch("selection_end", data.get("path"));
    }

    async function openDialog() {
        const path = await open({
            multiple: true,
            directory: true,
        }) as string|null;

        if (path == undefined) {
            return;
        }

        const input: HTMLInputElement = document.querySelector("input#path")!;
        input.value = path;
    }   
</script>

<section>
    <form on:submit={validateForm}>
        <p>Select the project folder.</p>
        <label for="path">Path: </label>
        <input id="path" type="text" name="path" readonly/>
        <button on:click|preventDefault={openDialog}>Select</button> 
        <button>Done</button>
    </form>
</section>

<style>
    section {
        display: flex;
        justify-content: center;
        width: 100%;
    }

    section form {
        position: absolute;
        background-color: rgb(169, 169, 169);
        color: rgb(36, 36, 36);
        padding: 20px;
        box-shadow: 1px 1px 5px;
    }

    p {
        margin-top: 0px;
    }

    input {
        text-overflow: ellipsis;
    }
</style>