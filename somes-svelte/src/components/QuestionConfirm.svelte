<script lang="ts">
	import type { Delegate } from "$lib/types";

	export let delegate: Delegate | undefined;
	export let question: string;
	export let details: string;
	export let disclaimerAcknowledged: boolean;
</script>

<div class="card w-full flex flex-col">
	<div class="p-4 flex flex-col gap-[2vh]">
		<p>Frage an: <br /><b>{delegate?.name}<br />{delegate?.party}</b></p>
		<p>Von: <br /><b>Max Mustermann</b></p>
		<p>Am: <br /><b>{new Date().toLocaleDateString()}</b> um <b>{new Date().toTimeString()}</b></p>
		<hr class="opacity-80" />
		<p>Titel: <br /><b>{question}</b></p>
		<hr class="opacity-80" />
		<p>Details: <br /><b>{details}</b></p>
	</div>
	<hr class="opacity-100 mt-auto" />
	<footer class="p-4 flex flex-row gap-4">
		<input class="checkbox w-12" type="checkbox" bind:checked={disclaimerAcknowledged} />
		Ich nehme zur Kenntnis, dass diese Frage unter dem von mir in meinem Somes-Benutzerprofil angegebenen
		Namen und mit der von mir angegebenen Email-Adresse an den/die Abgeordenete/n verschickt wird. Ich
		nehme auch zur Kenntnis, dass falls diese Frage unanständig ist, sie von der Somes-Fragenbank gelöscht
		werden kann.
	</footer>
</div>
