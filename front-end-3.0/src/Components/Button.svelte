<script>
    export let content
    export let handleClick
</script>

<style>
    .button {
        font-weight: bold;
        align-self: flex-end;
        background-color: #212121;
        border-radius: 10px;
        padding: 20px;
        margin: 20px;
        margin-top: 0px;
    }
</style>

<div class="button" on:click={handleClick}>{content}</div>
