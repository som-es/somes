<script lang="ts">
    import SButton from "$lib/components/UI/SButton.svelte";
    import { getModalStore, popup, type PopupSettings } from "@skeletonlabs/skeleton";

    const chatSocket = new WebSocket(import.meta.env.VITE_WEBSOCKET_URL);
    
    let input = "";
    
    const sendMessage = (input: string) => {
        if (!chatSocket || chatSocket.readyState !== WebSocket.OPEN) return;
        if ($modalStore.length > 0) {
            chatSocket.send(JSON.stringify({"question": input, "delegate_id": $modalStore[0].meta.delegate_id }));
        }
    }

    const recvMessage = (event: MessageEvent) => {
        console.log(event.data)
    }

    chatSocket.addEventListener("message", recvMessage);


    // sendMessage();

    const modalStore = getModalStore(); 
    export let parent;

    
    const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'extractedFromIntroductionVideo',
		placement: 'bottom'
	};
</script>
<div class="card p-8">
    <div class="!z-50 card p-4 w-72 shadow-xl" data-popup="extractedFromIntroductionVideo">
        <div class="z-50 font-bold text-xl">Hallo</div>
     </div>

    <!-- <div class="flex justify-between">
        <div></div> -->
    <button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable ">&#x2715</button
	>
	    <button class="text-4xl float-right"  use:popup={popupFeatured}>&#9432;</button>
    <!-- </div> -->
    <div class="mt-5">
        <input type="text" class="w-30 text-black" bind:value={input} />
        <SButton class=" bg-secondary-500" on:click={() => sendMessage(input)}>Senden</SButton>
    </div>
</div>

