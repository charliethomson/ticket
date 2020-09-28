<script>
    export let label
</script>

<style>
    label {
        font-weight: normal;
        font-size: 20px;
    }
    input {
        font-family: inherit;
        font-size: 1.3rem;
        font-weight: lighter;

        width: 100%;
        border: 0;
        outline: 0;
        background: transparent;
        border-bottom: 2px solid white;
    }
</style>

<label for="name">{label}:</label>
<input type="text" />
