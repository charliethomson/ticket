<script>
    import Note from "./Note.svelte"
    import { isNoteValid } from "../stores"

    export let workorder

    const notWhiteSpaceRegex = /^(?!\s*$).+/

    let currentNote = ""

    $: currentNoteValid = notWhiteSpaceRegex.exec(currentNote)

    function createNote() {
        if (currentNoteValid) {
            alert("Note created")
            $isNoteValid = true
        } else {
            $isNoteValid = false
        }
    }
</script>

<style>
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

<div class="container">
    <div class="create-note">
        <input
            type="text"
            placeholder="Enter notes here..."
            bind:value={currentNote}
            class={currentNoteValid ? '' : 'invalid '} />
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
</div>

<!-- TODO: Map the user id that I get from the API to an actual user -->
