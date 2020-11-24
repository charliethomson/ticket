<script>
    import { onMount } from "svelte"
    export let href = "/#"
    export let title
    export let alt
    let active

    const update = () => {
        let url = window.location.pathname + window.location.hash
        if (url == "/") url = "/#"
        active = url == href ? true : false
        if (active) document.title = alt ? alt : title
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
<a {href} class:active> {title}</a>
