<script lang="ts">
	import { jwtStore } from "$lib/caching/stores/stores";
	import Container from "$lib/components/Layout/Container.svelte";
	import SButton from "$lib/components/UI/SButton.svelte";
	import { getUserFromJwt, type BasicUserInfo, type QuizQuestion, type ScoreInfo, type Scorer } from "$lib/types";
    import { onDestroy, onMount } from "svelte";
	import { get } from "svelte/store";


    let enteredRoom = false;
    let recvUserCount = true;
    let userName: string | null = null
    let waitingForQuestions = false;
    let question: QuizQuestion | null = null;

    let currentScoreboard: Scorer[] | null = null;

    let currentScore: ScoreInfo | null = null;
    let userCount = 0;

    let state = "starting";
    let isAdmin = true;

    let jwtToken: string | null;

    onMount(async () => {
		jwtToken = get(jwtStore);
        if (!jwtToken) {
            return
        }
		const user = getUserFromJwt(jwtToken);
        isAdmin = user.is_admin;
	});
    
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

            if (!recvData) {
                question = null;
                return;
            }

            if (state == "scoreboard" && Array.isArray(recvData)) {
                currentScoreboard = recvData;
                console.log(currentScoreboard);
            } else if ("score" in recvData) {
                currentScore = recvData
            } else if ("question" in recvData) {
                question = recvData;
            } else if ("user_count" in recvData) {
                userCount = recvData.user_count
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
  
    setInterval(
		async () => {
            // if (isAdmin) sendMessage("u")
		},
        200
	);

    const enterRoom = () => {
        enteredRoom = true;
        state = "question";
        sendMessage("b")
    }

    const onScoreboard = () => {
        sendMessage("r"); 

        state = "scoreboard";
        sendMessage("s"); 
    }

</script>

{#if state == "starting"}
<div class="flex h-full items-center justify-center gap-4">

    <SButton class="bg-secondary-300" on:click={enterRoom} >Beitreten</SButton>

    {#if isAdmin}
        <!-- is admin -->
        <SButton class="bg-primary-300" on:click={() => {
            sendMessage(`h${jwtToken}`)
            waitingForQuestions = true;
            state = "question";
        }} >Admin</SButton>
    {/if}


</div>

{/if}
 
<Container>
    {#if isAdmin}
        <div class="flex justify-between">
            <div></div>
        <!-- is admin -->
        {#if state == "question"}
            <SButton on:click={onScoreboard}>Weiter</SButton>
        {:else if state == "scoreboard"}
            <SButton on:click={() => {
                sendMessage("n"); 
                state = "question";
            }} >Nächste Frage</SButton>
        {/if}
        </div>
    {/if}

    {#if userName}
        {userName}
    {/if}

    {#if question}
        <header>
            <h1 class=" font-bold text-6xl">
                {question.question}
            </h1>
        </header>

        <div class="content">
            <div class="squares-container">
                <div on:click={() => sendMessage("a1")} on:keydown role="button" tabindex=0 class="square bg-primary-400 flex justify-center items-center text-center text-xl">{question.answer1}</div>
                <div on:click={() => sendMessage("a2")} on:keydown role="button" tabindex=0 class="square bg-secondary-400 flex justify-center items-center text-center text-xl"> {question.answer2}</div>
                <div on:click={() => sendMessage("a3")} on:keydown role="button" tabindex=0 class="square bg-tertiary-500 flex justify-center items-center text-center text-xl">{question.answer3}</div>
                <div on:click={() => sendMessage("a4")} on:keydown role="button" tabindex=0 class="square bg-surface-500 flex justify-center items-center text-center text-xl text-white">{question.answer4}</div>
            </div>
        </div>
    {/if}

</Container>


<style>
header {
  text-align: center;
  margin-bottom: 40px;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.squares-container {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  justify-items: center;
}

.square {
  width: 100%;
  height: 200px;
  border-radius: 8px;
}

@media (max-width: 768px) {
  .squares-container {
    grid-template-columns: repeat(2, 1fr);
  }

  .square {
    height: 150px;
  }

  .content {
    gap: 20px;
  }
}

@media (max-width: 480px) {
  .squares-container {
    grid-template-columns: 1fr;
  }

  .square {
    height: 150px;
  }

  .content {
    gap: 10px;
  }
}
 
</style>