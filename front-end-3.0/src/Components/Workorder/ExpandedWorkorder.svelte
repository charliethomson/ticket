<script>
    import { workorders } from '../../sampleData'
    import Container from "../Helpers/Container.svelte"
    import Note from "./Expanded/Note.svelte"
    import Location from "../Statuses/Location.svelte"
    import Statuses from "../Statuses/Statuses.svelte"
    import { alertContent } from "../../stores"
    export let statuses = []
    export let travelStatuses = []

    let id = getWorkorderID()
    let workorder = workorders[id]

    // TODO: get data from API
    let notes = [
        {
            user: {
                first_name: "Charlie",
                last_name: "Thomson",
            },
            contents:
                "this is a workorder note, this is a workorder note, this is a workorder note, this is a workorder note",
            created: 1605187812,
        },
    ]

    let currentNote = {
        user: {
            first_name: "Charlie",
            last_name: "Thomson",
        },
        contents: "",
        created: 1605187812,
        workorder_id: workorder.workorder_id,
    }
    const notWhiteSpaceRegex = /^(?!\s*$).+/
    $: currentNoteValid = notWhiteSpaceRegex.test(currentNote.contents)

    async function createNote() {
        alert("Note created")
        if (currentNoteValid) {
            // const response = await fetch("http://offsite.repair/api/notes", {
            //     method: "POST",
            //     mode: "cors",
            //     headers: {
            //         "Content-Type": "application/json",
            //     },
            //     body: JSON.stringify(currentNote),
            // })
            // const res = await response.json()
            // return res
            notes.push(currentNote)
            notes = notes
            $alertContent = ""
            currentNote.contents = ""
            console.log(notes)
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
    //OnMount this component makes a request to the API for the entire workorder
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
                    createNote()
                }
            }} />
        <div class="button" on:click={createNote}>Create note</div>
    </div>
    <div class="notes">
        {#each notes.reverse() as note}
            <Note
                name={note.user.first_name + ' ' + note.user.last_name}
                date={note.created}
                notes={note.contents} />
        {/each}
    </div>
</Container>

<!-- TODO: Map the user id that I get from the API to an actual user -->
