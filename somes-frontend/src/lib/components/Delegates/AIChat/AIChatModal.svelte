<script lang="ts">
	import { onDestroy } from 'svelte';
	import type { Delegate } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';

	let isGenerating = false;

	const chatSocket = new WebSocket(import.meta.env.VITE_WEBSOCKET_URL);

	onDestroy(() => {
		chatSocket.close();
	});

	export let delegate: Delegate;

	let messages: any[] = [];
	messages = [
		{
			role: 'assistant',
			content: `Ich bin SomBOT, ein Chatbot des Demokratieprojekts "somes". Ich bin spezialisiert darauf, Fragen über ${delegate.name} (${delegate.party}) zu verschiedenen Themen zu beantworten.`
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

		if (newMessage.length > 0) {
			const chatHistory = messages.slice();
			messages = [
				...messages,
				{ role: 'user', content: sentMessage },
				{ role: 'assistant', content: '' }
			];
			chatSocket.send(
				JSON.stringify({
					question: sentMessage,
					delegate_id: delegate.id,
					chat_history: chatHistory
				})
			);

			isGenerating = true;
			newMessage = '';
		}
	};
</script>

<div
	class="flex flex-col justify-between w-full max-w-7xl h-[90vh] bg-primary-100-900 shadow-lg rounded-lg overflow-hidden"
>
	<div class="p-4 bg-primary-300 text-center  items-center text-lg font-bold flex justify-between">
		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<span class="text-4xl">⚠</span>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content
					class="z-90 text-sm w-72 p-4 bg-primary-100 dark:bg-primary-600 rounded-lg shadow-lg"
				>
					Die Antworten des Chatbots basieren auf Ausschnitten von Reden der jeweiligen Person. Diese
					Ausschnitte können unvollständig oder aus dem Kontext gerissen sein, was zu ungenauen oder
					irreführenden Antworten führen kann. Bitte beachten Sie, dass der Chatbot nicht die tatsächlichen
					Meinungen oder Aussagen der Person widerspiegelt.
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<!-- <Popover title="Hinweis" placement="bottom" trigger="hover"  transitionParams={{ duration: 200 }} class="z-40 text-sm w-72 p-4">
			Die Antworten des Chatbots basieren auf Ausschnitten von Reden der jeweiligen Person. Diese
			Ausschnitte können unvollständig oder aus dem Kontext gerissen sein, was zu ungenauen oder
			irreführenden Antworten führen kann. Bitte beachten Sie, dass der Chatbot nicht die tatsächlichen
			Meinungen oder Aussagen der Person widerspiegelt.
		</Popover> -->
		<div>AI Chat</div>
		 <Dialog.Close>
			<button class="text-5xl">✕</button>
		 </Dialog.Close>
		<!-- <X /> -->
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
		class="flex items-center border-t border-gray-200 dark:border-gray-700 p-4 bg-primary-100 dark:bg-gray-800"
	>
		<input
			type="text"
			bind:value={newMessage}
			placeholder="Stelle deine Frage..."
			on:keypress={(e) => e.key === 'Enter' && sendMessage()}
			class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-full text-base outline-hidden focus:ring-3 focus:ring-primary text-black"
		/>
		<button
			on:click={sendMessage}
			class="ml-4 px-4 py-2 bg-primary-500 text-white rounded-full hover:bg-primary-800 focus:outline-hidden focus:ring-3 focus:ring-primary-dark"
			>Senden</button
		>
	</div>
</div>
