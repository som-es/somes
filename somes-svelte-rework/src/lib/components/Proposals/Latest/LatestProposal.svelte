<script lang="ts">
	import type { GovProposalDelegate } from '$lib/types';
	import GovProposalExpandableBar from './GovProposalExpandableBar.svelte';

	export let govProposal: GovProposalDelegate;
</script>

<GovProposalExpandableBar {govProposal} />
