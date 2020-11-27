<script>
    import Nav from "./Components/Nav/Nav.svelte"
    import Alert from "./Components/Helpers/Alert.svelte"
    import Route from "./Components/Helpers/Route.svelte"
    import { alertContent } from "./stores"
    import CreateWorkorder from "./Components/Pages/CreateWorkorder.svelte"
    import HomePage from "./Components/Pages/HomePage.svelte"
    import Workorder from "./Components/Pages/Workorder.svelte"
</script>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Montserrat");

    :global(html, body) {
        height: 100%;
        background-color: #121212;
    }
    :global(button) {
        border: none;
    }
    :global(*) {
        color: #e3e3e3;
        font-family: "Montserrat", sans-serif;
    }
    main {
        height: 100%;
        display: flex;
        flex-direction: column;
    }
</style>

<main>
    <Nav />
    <Route component={HomePage} href={'/'} />
    <Route component={Workorder} href={'/workorder'} />
    <Route component={CreateWorkorder} href={'/create-workorder'} />
</main>

{#if $alertContent}
    <Alert>{$alertContent}</Alert>
{/if}

<!-- TODO: Make login button with a tag that links to /api/auth/login -->
