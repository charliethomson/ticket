<script>
    import { onMount } from "svelte"
    import { activeWorkorder } from "../../stores"

    export let href = "/#"
    export let id = 0
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
<a
    {href}
    class:active
    on:click={(e) => {
        $activeWorkorder = id
    }}>
    <slot />
</a>
