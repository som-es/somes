<script lang="ts">
	import SButton from "$lib/components/UI/SButton.svelte";
import { onDestroy } from "svelte";

    
    const recvMessage = (event: MessageEvent) => {
        console.log(event)
	};

    
	const roomSocket = new WebSocket(import.meta.env.VITE_ROOM_WEBSOCKET_URL);

    onDestroy(() => {
		roomSocket.close();
	});
	roomSocket.addEventListener('message', recvMessage);

    const sendMessage = (msg: string) => {
		if (!roomSocket || roomSocket.readyState !== WebSocket.OPEN) return;
        roomSocket.send(msg)
    }

</script>

<div>
    <SButton on:click={() => sendMessage("b")} >Beitreten</SButton>
    <SButton on:click={() => sendMessage("n")} >Nächste Frage</SButton>
</div>