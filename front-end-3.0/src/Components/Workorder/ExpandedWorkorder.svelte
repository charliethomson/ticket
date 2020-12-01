<script>
    import { onMount } from "svelte"
    import { getNotes, getUsers, createNote } from "../../utils"
    import Container from "../Helpers/Container.svelte"
    import Note from "./Expanded/Note.svelte"
    import Location from "../Statuses/Location.svelte"
    import Statuses from "../Statuses/Statuses.svelte"
    import { alertContent } from "../../stores"
    export let statuses = []
    export let travelStatuses = []

    let notes = []
    let users = []

    let id = 0

    // FIXME: make sure POST request is using the correct object values for prod
    let currentNote = {
        user: 0,
        contents: "",
        created: 1605187812,
    }
    const notWhiteSpaceRegex = /^(?!\s*$).+/
    $: currentNoteValid = notWhiteSpaceRegex.test(currentNote.contents)

    async function create() {
        if (currentNoteValid) {
            notes = [...notes, await createNote(currentNote)]
            currentNote.contents = ""
            $alertContent = ""
        } else {
            $alertContent = "Please check your note!"
        }
    }

    function getWorkorderID() {
        let hash = window.location.hash
        if (hash.includes("/workorder")) {
            let parts = hash.split("/")
            let id = parts[2]
            return id
        }
        return -1
    }

    onMount(async () => {
        id = getWorkorderID()
        // TODO: backend: notes should accept a workorder ID and only return the notes for that workorder
        notes = await getNotes(id)
            .then((data) => data.reverse())
            .catch((err) => [])
        users = await getUsers()
            .then((data) => data)
            .catch((err) => [])
    })
</script>

<style>
    .statuses {
        margin-bottom: 20px;
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
    }
    .create-note {
        display: flex;
        align-items: center;
    }
    input {
        font-family: "Montserrat", sans-serif;
        padding: 10px;
        height: 50px;
        border-radius: 10px;
        border: none;
        outline: none;
        color: #e3e3e3;
        background: transparent;
        flex-grow: 1;
        margin-right: 20px;
    }
    input:focus {
        border: 2px solid #e3e3e3;
    }

    .valid:focus {
        border: 2px solid #388e3c;
    }

    .invalid:focus {
        border: 2px solid #f44336;
    }

    .button {
        font-weight: bold;
        cursor: pointer;
        padding: 10px;
        border-radius: 10px;
        background-color: #388e3c;
    }
</style>

<div class="statuses">
    <Statuses {statuses} {id} />
    <Location {travelStatuses} {id} />
</div>

<Container>
    <div class="create-note">
        <input
            type="text"
            placeholder="Enter notes here..."
            bind:value={currentNote.contents}
            class={currentNoteValid ? 'valid' : 'invalid '}
            on:keydown={(e) => {
                if (e.code === 'Enter') {
                    create()
                }
            }} />
        <div class="button" on:click={create}>Create note</div>
    </div>
    <div class="notes">
        {#if notes != [] && users != []}
            {#each notes as note}
                {#if users[note.user]}
                    <Note
                        name={users[note.user].first_name + ' ' + users[note.user].last_name}
                        date={note.created}
                        notes={note.contents} />
                {/if}
            {/each}
        {:else}Loading notes...{/if}
    </div>
</Container>
