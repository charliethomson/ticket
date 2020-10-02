<script>
    import Note from "./Note.svelte"
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
        color: white;
        background: transparent;
        flex-grow: 1;
        margin-right: 20px;
    }
    .button {
        padding: 10px;
        border-radius: 10px;
        background-color: #45bc45;
    }
</style>

<div class="container">
    <div class="create-note">
        <input type="text" placeholder="Enter notes here..." />
        <div class="button">Create note</div>
    </div>
    <div class="notes">
        <Note
            name="Colton Hobbs"
            date="May 30 @ 11:29AM"
            notes="Device is a MacBook Pro (A1706) that needs a screen replacement. Quoting 3-5 business days for the part, $549.99, and EOD when the part arrives. Fully functional otherwise. -Sam" />
        <Note
            name="Colton Hobbs"
            date="May 30 @ 11:29AM"
            notes="Device is a MacBook Pro (A1706) that needs a screen replacement. Quoting 3-5 business days for the part, $549.99, and EOD when the part arrives. Fully functional otherwise. -Sam, Device is a MacBook Pro (A1706) that needs a screen replacement. Quoting 3-5 business days for the part, $549.99, and EOD when the part arrives. Fully functional otherwise. -Sam" />
    </div>
</div>
