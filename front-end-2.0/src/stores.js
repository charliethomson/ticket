import { writable } from "svelte/store"

export let toggleCreate = writable(false)
export let toggleSearch = writable(true)
