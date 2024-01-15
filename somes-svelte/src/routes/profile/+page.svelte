<script lang="ts">
	import { goto } from "$app/navigation";
	import { get } from "svelte/store";
	import { jwtStore, userStore } from "../../stores/stores";
	import { t } from "$lib/translations";
	import { browser } from "$app/environment";

	const user = get(userStore);

	if (!user) {
		$: if (browser) {
  			goto("/")
		}
	}
</script>

<div>
	<input
		value={$t("common.logout")}
		type="button"
		on:click={async () => {
			userStore.set(null);
			jwtStore.set(null);
			$: if (browser) {
				goto("/")
			}
		}}
	/>
</div>

<style>
	input[type="button"] {
		/* background-color: #5c5cd6; */
		background-color: rgb(var(--color-tertiary-500));
		color: #fff;
		padding: 10px;
		border: none;
		border-radius: 5px;
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
		font-size: 20px;
		cursor: pointer;
		transition: all 0.3s ease;
	}
</style>
