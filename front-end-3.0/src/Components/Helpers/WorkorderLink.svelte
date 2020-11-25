<script>
    import { onMount } from "svelte"

    export let href = "/#/"
    export let alt = ""
    let active = false
    let dest = ""

    const update = () => {
        dest = "/#" + href
        let url = window.location.pathname + window.location.hash
        if (url == "/" || url == "/#") url = "/#/"
        active = url == dest ? true : false
        if (active) document.title = alt ? alt : document.title
    }

    onMount(update)
</script>

<style>
    a {
        color: inherit;
        text-decoration: inherit;

        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid #e3e3e3;
        padding: 10px;
        cursor: pointer;
    }
    /* a > {
        width: 200px;
        text-align: center;
    } */
</style>

<svelte:window on:hashchange={update} />
<a href={dest} class:active><slot /></a>
