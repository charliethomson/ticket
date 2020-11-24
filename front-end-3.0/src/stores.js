import { writable } from "svelte/store"


export let activeWorkorder = writable(0)
export let alertContent = writable('')
export let isFormValid = writable(true)
