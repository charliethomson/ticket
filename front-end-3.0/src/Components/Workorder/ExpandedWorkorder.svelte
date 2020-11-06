<script>
    import Container from "../Container.svelte"
    import Note from "./Expanded/Note.svelte"
    import Location from "../Statuses/Location.svelte"
    import Statuses from "../Statuses/Statuses.svelte"

    export let workorder
    export let statuses
    export let travelStatuses

    const notWhiteSpaceRegex = /^(?!\s*$).+/

    let currentNote = ""

    $: currentNoteValid = notWhiteSpaceRegex.exec(currentNote)

    function createNote() {
        alert("Note created")
        // if (currentNoteValid) {
        //     alert("Note created")
        //     $isNoteValid = true
        // } else {
        //     $isNoteValid = false
        // }
    }
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
    <Statuses {workorder} {statuses} />
    <Location {travelStatuses} {workorder} />
</div>

<Container>
    <div class="create-note">
        <input
            type="text"
            placeholder="Enter notes here..."
            bind:value={currentNote}
            class={currentNoteValid ? 'valid' : 'invalid '} />
        <div class="button" on:click={createNote}>Create note</div>
    </div>
    <div class="notes">
        {#each workorder.notes as note}
            <Note
                name={note.user.first_name + ' ' + note.user.last_name}
                date={note.created}
                notes={note.contents} />
        {/each}
    </div>
</Container>

<!-- TODO: Map the user id that I get from the API to an actual user -->
