<script>
    export let label
    let content
</script>

<style>
    label {
        font-weight: normal;
        font-size: 15px;
    }
    input {
        font-family: inherit;
        font-weight: lighter;
        color: white;

        width: 100%;
        border: 0;
        outline: 0;
        background: transparent;
        border-bottom: 2px solid white;
        margin-bottom: 10px;
    }
</style>

<label for="name">{label}:</label>
<input type="text" bind:value={content} />
