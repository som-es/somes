import { persisted } from "./persisted.svelte";

export const lightModeStore = persisted<string>('theme', 'light');