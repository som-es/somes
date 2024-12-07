<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';
    let messages = [
        { type: 'received', content: 'Hello! How can I help you today?' },
        { type: 'sent', content: 'Hi! Can you tell me more about your services?' },
        { type: 'received', content: 'Of course! We offer a range of solutions tailored to your needs.' }
    ];
    let newMessage = '';

    const sendMessage = () => {
        if (newMessage.trim()) {
            messages = [...messages, { type: 'sent', content: newMessage.trim() }];
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
    <div class="z-50 font-bold text-xl">Hallo</div>
</div>
<div class="flex flex-col justify-between w-full max-w-7xl h-[90vh] bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden">
    <div class="p-4 bg-primary text-white text-center text-lg font-bold flex justify-between">
        <div>
            Chat
        </div>

        <div>
            Chat
        </div>
	    <button class="text-4xl float-right"  use:popup={popupFeatured}>&#9432;</button>
    </div>
    <div class="flex-1 p-4 overflow-y-auto bg-gray-50 dark:bg-gray-900">
        {#each messages as { type, content }}
            <div class={`flex mb-4 ${type === 'sent' ? 'justify-end' : 'justify-start'}`}>
                <div class={`max-w-[70%] px-4 py-2 rounded-lg text-sm ${type === 'sent' ? 'bg-secondary-500 text-white' : 'bg-primary-400 text-gray-900 dark:text-gray-100'}`}>{content}</div>
            </div>
        {/each}
    </div>
    <div class="flex items-center border-t border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
        <input type="text" bind:value={newMessage} placeholder="Type your message here..." on:keypress={(e) => e.key === 'Enter' && sendMessage()} class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-full text-base outline-none focus:ring focus:ring-primary text-black" />
        <button on:click={sendMessage} class="ml-4 px-4 py-2 bg-primary-500 text-white rounded-full hover:bg-primary-800 focus:outline-none focus:ring focus:ring-primary-dark">Send</button>
    </div>
</div>