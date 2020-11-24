<script>
    import { onMount } from "svelte"

    export let component = null
    export let href = "/#"
    let route

    const clean = (s) => JSON.stringify(s.split("/").filter((s) => s))

    const render = () => {
        let url = window.location.pathname + window.location.hash
        if (url == "/") url = "/#"

        route = clean(href) == clean(url) ? component : null
    }

    onMount(render)
</script>

<svelte:window on:hashchange={render} />
<svelte:component this={route} {...$$props} />
