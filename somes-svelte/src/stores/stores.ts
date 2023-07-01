import type { Writable } from "svelte/store";
import { localStorageStore } from '@skeletonlabs/skeleton';

export let jwtStore: Writable<string | null> = localStorageStore("jwt", null);