<script>
    import { onMount } from "svelte"
    export let href = "/"
    export let alt = ""
    let active
    let dest

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
        padding: 25px;
        display: block;
        font-weight: bold;

        color: inherit;
        text-decoration: inherit;
    }
    a:hover {
        background-color: #121212;
    }
</style>

<svelte:window on:hashchange={update} />
<a href={dest} class:active><slot /></a>
