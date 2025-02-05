<script lang="ts">
	import SButton from "$lib/components/UI/SButton.svelte";
	import type { QuizQuestion } from "$lib/types";
    import { onDestroy } from "svelte";


    let enteredRoom = false;
    let userName: string | null = null
    let waitingForQuestions = false;
    let question: QuizQuestion | null = null;

    let currentScore: number = 0;
    let currentPlace: number | null = null
    
    const recvMessage = (event: MessageEvent) => {

        console.log(event)
        if (enteredRoom) {
            const data = event.data as string;
            userName = data.slice(0, data.indexOf(";"))

            enteredRoom = false;
            waitingForQuestions = true

            return
        }
        if (waitingForQuestions) {

            const recvData: any = JSON.parse(event.data as string)

            if ("score" in recvData) {

            } else if ("answer1" in recvData) {
                question = recvData
            } else {
                question = null
            }
        }
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

    const enterRoom = () => {
        enteredRoom = true;
        sendMessage("b")
    }

</script>

<div>
    <SButton on:click={enterRoom} >Beitreten</SButton>
    <SButton on:click={() => sendMessage("n")} >Nächste Frage</SButton>
</div>

{#if userName}
    {userName}
{/if}


{#if question}
    {question.question}

    <div class="flex flex-col">
        <SButton on:click={() => sendMessage("a1")} >{question.answer1}</SButton>
        <SButton on:click={() => sendMessage("a2")} >{question.answer2}</SButton>
        <SButton on:click={() => sendMessage("a3")} >{question.answer3}</SButton>
        <SButton on:click={() => sendMessage("a4")} >{question.answer4}</SButton>
    </div>
{/if}