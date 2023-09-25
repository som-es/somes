import type { Writable } from "svelte/store";
import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Delegate, UserInfo } from "$lib/types";

export let jwtStore: Writable<string | null> = localStorageStore("jwt", null);
export let userStore: Writable<UserInfo | null> = localStorageStore("user", null);
export let verificationMailStore: Writable<string | null> = localStorageStore(
	"verificationMail",
	null,
);
export let modalDelegateStore: Writable<Delegate | undefined> = localStorageStore("modalDelegate", undefined);
