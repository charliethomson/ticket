<script>
    import { workorderExpanded } from "../stores";
    import Note from "./Note.svelte";

    function createNote() {
        console.log("Note created");
    }

    async function getWorkorder() {
        return await fetch(
            `http://localhost:8080/api/workorders?id=${$workorderExpanded}`,
            { mode: "cors" }
        )
            .then(async (response) => await response.json())
            .then((json) => {
                if (json.ok) {
                    return json.message[0];
                } else {
                    throw "Got err response from server: ${json.message}";
                }
            })
            .then((message) => {
                console.log(message);
                return message;
            })
            .catch((e) => console.error(e));
    }

    console.log(getWorkorder());
</script>

<style>
    .create-note {
        display: flex;
        align-items: center;
    }
    input {
        font-family: "Montserrat", sans-serif;
        height: 50px;
        border-radius: 10px;
        border: none;
        color: #e3e3e3;
        background: transparent;
        flex-grow: 1;
        margin-right: 20px;
    }
    .button {
        font-weight: bold;
        cursor: pointer;
        padding: 10px;
        border-radius: 10px;
        background-color: #388e3c;
    }
</style>

<div class="container">
    <div class="notes">
        {#await getWorkorder()}
            <p>loading</p>
        {:then workorderContent}
            <div class="create-note">
                <input type="text" placeholder="Enter notes here..." />
                <div class="button" on:click={createNote}>Create note</div>
            </div>

            {#each workorderContent.notes as { userId, contents, created, next_update }}
                <Note
                    user={userId}
                    {created}
                    {contents}
                    next_udpate={next_update} />
            {/each}
        {/await}
    </div>
</div>
