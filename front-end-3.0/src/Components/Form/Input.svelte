<script>
    export let label = ""
    export let value = ""
    export let name = ""
    export let valid = value => false
    export let maxlength = 524288
    export let size = 20
</script>

<style>
    input {
        margin-top: 10px;
        margin-bottom: 10px;
        font-family: inherit;
        color: white;
        padding: 10px;
        background: transparent;
        outline: 0;
        width: 100%;

        border: 2px solid #e3e3e3;
        border-radius: 10px;
    }
    .invalid:focus {
        border: 2px solid red;
    }
    .valid:focus {
        border: 2px solid #388e3c;
    }
</style>

<label for={name}>{label}: </label>
<input
    {name}
    bind:value
    type="text"
    class={valid(value) ? 'valid' : 'invalid '}
    {maxlength}
    {size} />