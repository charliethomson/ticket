<script>
    import { onMount } from "svelte"
    export let component = null
    export let href = ""
    let route = null

    const render = () => {
        let url = window.location.pathname + window.location.hash
        if (url == "/" || url == "/#") {
            url = "/#/"
            window.location = "/#/"
        }

        if (href != "/" || url == "/#/") {
            let dest = "/#" + href
            let show = url.includes(dest)
            route = show ? component : null 
        } else {
            route = null
        }
    }

    onMount(render)
</script>

<svelte:window on:hashchange={render} />
<svelte:component this={route} />
