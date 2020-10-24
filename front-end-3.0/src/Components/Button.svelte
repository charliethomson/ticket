<script>
    export let content
    export let handleClick
</script>

<style>
    .button {
        align-items: center;
        font-weight: bold;
        align-self: flex-end;
        padding: 25px;
    }
    .button:hover {
        background-color: #121212;
    }
</style>

<div class="button" on:click={handleClick}>{content}</div>
