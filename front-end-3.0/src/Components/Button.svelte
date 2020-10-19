<script>
    export let content
    export let handleClick
    export let handleHover
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

<div class="button" on:click={handleClick} on:mouseover={handleHover}>
    {content}
</div>
