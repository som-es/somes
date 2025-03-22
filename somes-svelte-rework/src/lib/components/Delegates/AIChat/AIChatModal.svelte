<script lang="ts">
	import SButton from '$lib/components/UI/SButton.svelte';
	import { getModalStore, popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import Chat from './Chat.svelte';
	import { onDestroy } from 'svelte';
	import Warning from '$lib/components/UI/Warning.svelte';

	let isGenerating = false;

	const chatSocket = new WebSocket(import.meta.env.VITE_WEBSOCKET_URL);

	onDestroy(() => {
		chatSocket.close();
	});

	const modalStore = getModalStore();
	export let parent;

	// $modalStore[0].meta.
	let messages: any[] = [];
	if ($modalStore.length > 0)
		messages = [
			{
				role: 'assistant',
				content: `Ich bin SomBOTka, ein Chatbot vom Demokratieprojekt "somes". Ich bin spezialisiert darauf, Fragen über ${$modalStore[0].meta.delegate.name} (${$modalStore[0].meta.delegate.party}) zu verschiedenen Themen zu beantworten.`
			}
		];

	const recvMessage = (event: MessageEvent) => {
		// console.log(event.data)
		if (event.data.includes('[END]')) {
			isGenerating = false;
		} else {
			messages[messages.length - 1].content += event.data;
			messages = messages;
		}
	};

	chatSocket.addEventListener('message', recvMessage);

	let newMessage = '';

	const sendMessage = () => {
		if (isGenerating) {
			return;
		}
		const sentMessage = newMessage.trim();
		if (!sentMessage || !chatSocket || chatSocket.readyState !== WebSocket.OPEN) return;

		if ($modalStore.length > 0 && newMessage.length > 0) {
			const chatHistory = messages.slice();
			messages = [
				...messages,
				{ role: 'user', content: sentMessage },
				{ role: 'assistant', content: '' }
			];
			chatSocket.send(
				JSON.stringify({
					question: sentMessage,
					delegate_id: $modalStore[0].meta.delegate.id,
					chat_history: chatHistory
				})
			);

			isGenerating = true;
			newMessage = '';
		}
	};

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'extractedFromIntroductionVideo',
		placement: 'bottom'
	};
</script>

<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="extractedFromIntroductionVideo">
	<div class="z-50 font-bold text-xl">
		Die Fragen werden an ein self-hosted LLM gesendet und nicht an die jeweilige Person. Als Basis
		für die Informationen werden Auschnitte aus den gehaltenen Reden genommen.
	</div>
</div>
<div
	class="flex flex-col justify-between w-full max-w-7xl h-[90vh] bg-primary-100-800-token shadow-lg rounded-lg overflow-hidden"
>
	<div class="p-4 bg-primary text-center text-lg font-bold flex justify-between">
		<button class="text-4xl" use:popup={popupFeatured}>⚠</button>
		<div>Chat</div>

		<button
			on:click={() => {
				modalStore.close();
			}}
			style="font-size: 34px"
			class="w-5 unselectable"
		>
			&#x2715
		</button>
	</div>
	<div class="flex-1 p-4 overflow-y-auto bg-gray-50 dark:bg-gray-900">
		{#each messages as { role, content }}
			<div class={`flex mb-4 ${role === 'user' ? 'justify-end' : 'justify-start'}`}>
				<div
					class={`max-w-[70%] px-4 py-2 rounded-lg text-sm ${role === 'user' ? 'bg-secondary-500 text-white' : 'bg-primary-400 text-gray-900 dark:text-gray-100'}`}
				>
					{content}
				</div>
			</div>
		{/each}
	</div>
	<div
		class="flex items-center border-t border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800"
	>
		<input
			type="text"
			bind:value={newMessage}
			placeholder="Stelle deine Frage..."
			on:keypress={(e) => e.key === 'Enter' && sendMessage()}
			class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-full text-base outline-none focus:ring focus:ring-primary text-black"
		/>
		<button
			on:click={sendMessage}
			class="ml-4 px-4 py-2 bg-primary-500 text-white rounded-full hover:bg-primary-800 focus:outline-none focus:ring focus:ring-primary-dark"
			>Senden</button
		>
	</div>
</div>
