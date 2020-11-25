import { writable } from "svelte/store"

export let alertContent = writable('')
export let isFormValid = writable(true)
