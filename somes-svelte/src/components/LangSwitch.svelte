<script lang="ts">
	import { RadioGroup, RadioItem } from "@skeletonlabs/skeleton";
	import { Icon } from "svelte-flag-icons";
	import { t, locale, locales } from "$lib/translations";

	const handleChange = ({ currentTarget }: { currentTarget: any }) => {
		const { value } = currentTarget;

		document.cookie = `lang=${value}`;
	};
</script>

<RadioGroup rounded="rounded-container-token" padding="px-2">
	<RadioItem bind:group={$locale} name="justify" value={"de"} on:change={handleChange}>
		<Icon name="at" />
	</RadioItem>
	<RadioItem bind:group={$locale} name="justify" value={"en"} on:change={handleChange}>
		<Icon name="gb" />
	</RadioItem>
</RadioGroup>
