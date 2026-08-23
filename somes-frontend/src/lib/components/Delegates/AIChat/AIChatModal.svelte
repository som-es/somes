<script lang="ts">
	import { onDestroy } from 'svelte';
import { t } from '$lib/i18n/i18n.svelte';
	import type { Delegate } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

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
			content: t('aiChat.intro', { name: delegate.name, party: delegate.party })
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
	class="flex h-[90vh] w-full max-w-7xl flex-col justify-between overflow-hidden rounded-lg bg-primary-100-900 shadow-lg"
>
	<div class="flex items-center justify-between bg-primary-300 p-4 text-center text-lg font-bold">
		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<span class="text-4xl">⚠</span>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content
					class="z-90 w-72 rounded-lg bg-primary-100 p-4 text-sm shadow-lg dark:bg-primary-600"
				>
					t('aiChat.disclaimer.part1')
					t('aiChat.disclaimer.part2')
					t('aiChat.disclaimer.part3')
					t('aiChat.disclaimer.part4')
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<!-- <Popover title="Hinweis" placement="bottom" trigger="hover"  transitionParams={{ duration: 200 }} class="z-40 text-sm w-72 p-4">
			t('aiChat.disclaimer.part1') Diese
			Ausschnitte können unvollständig oder aus dem Kontext gerissen sein, was zu ungenauen oder
			irreführenden Antworten führen kann. Bitte beachten Sie, dass der Chatbot nicht die tatsächlichen
			Meinungen oder Aussagen der Person widerspiegelt.
		</Popover> -->
		<div>AI Chat</div>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
		<!-- <X /> -->
	</div>
	<div class="flex-1 overflow-y-auto bg-gray-50 p-4 dark:bg-gray-900">
		{#each messages as { role, content }}
			<div class={`mb-4 flex ${role === 'user' ? 'justify-end' : 'justify-start'}`}>
				<div
					class={`max-w-[70%] rounded-lg px-4 py-2 text-sm ${role === 'user' ? 'bg-secondary-500 text-white' : 'bg-primary-400 text-gray-900 dark:text-gray-100'}`}
				>
					{content}
				</div>
			</div>
		{/each}
	</div>
	<div
		class="flex items-center border-t border-gray-200 bg-primary-100 p-4 dark:border-gray-700 dark:bg-gray-800"
	>
		<input
			type="text"
			bind:value={newMessage}
			placeholder="Stelle deine Frage..."
			on:keypress={(e) => e.key === 'Enter' && sendMessage()}
			class="focus:ring-primary flex-1 rounded-full border border-gray-300 px-4 py-2 text-base outline-hidden focus:ring-3 dark:border-gray-600"
		/>
		<button
			on:click={sendMessage}
			class="focus:ring-primary-dark ml-4 rounded-full bg-primary-500 px-4 py-2 text-white hover:bg-primary-800 focus:ring-3 focus:outline-hidden"
			>Senden</button
		>
	</div>
</div>
