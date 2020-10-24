<script>
    import Note from "./Note.svelte"

    export let workorder

    function createNote() {
        alert("Note created")
    }
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
    <div class="create-note">
        <input type="text" placeholder="Enter notes here..." />
        <div class="button" on:click={createNote}>Create note</div>
    </div>
    <div class="notes">
        {#each workorder.notes as note}
            <Note name={note.user} date={note.created} notes={note.contents} />
        {/each}
    </div>
</div>

<!-- TODO: Map the user id that I get from the API to an actual user in some kind of model -->
