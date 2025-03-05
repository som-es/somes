<script lang="ts">
	import { jwtStore } from '$lib/caching/stores/stores';
	import Container from '$lib/components/Layout/Container.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import {
		getUserFromJwt,
		type BasicUserInfo,
		type InfoCounts,
		type QuizQuestion,
		type ScoreInfo,
		type Scorer
	} from '$lib/types';
	import { setModeCurrent } from '@skeletonlabs/skeleton';
	import { onDestroy, onMount } from 'svelte';
	import { get } from 'svelte/store';

	let enteredRoom = false;
	let recvUserCount = true;
	let userName: string | null = null;
	let userId: string | null = null;
	let waitingForQuestions = false;
	let prevQuestion: QuizQuestion | null = null;
	let question: QuizQuestion | null = null;

    setModeCurrent(true);

	let currentScoreboard: Scorer[] | null = null;

	let prevScore: ScoreInfo | null = null;
	let currentScore: ScoreInfo | null = null;
	let infoCounts: InfoCounts | null = null;

	let state = 'starting';
	let isAdmin = false;

	let jwtToken: string | null;
	let selectedAnswer: number | null = null;

	const CURRENT_QUESTION_TIME = 18;

	let currentQuestionTimeLeftSeconds = CURRENT_QUESTION_TIME;

	onMount(async () => {
		jwtToken = get(jwtStore);
		if (!jwtToken) {
			return;
		}
		const user = getUserFromJwt(jwtToken);
		isAdmin = user.is_admin;
	});

	const recvMessage = (event: MessageEvent) => {
		if (enteredRoom && !waitingForQuestions) {
			const data = event.data as string;
			userName = data.slice(0, data.indexOf(';'));
			userId = data.slice(data.indexOf(';') + 1);

			waitingForQuestions = true;

			return;
		}
		if (waitingForQuestions) {
			const recvData: any = JSON.parse(event.data as string);

			if (!recvData) {
				question = null;
				return;
			}

			if (state == 'scoreboard' && Array.isArray(recvData)) {
				currentScoreboard = recvData;
				console.log(currentScoreboard);
			} else if ('score' in recvData) {
				currentScore = recvData;
				console.log(currentScore);
                console.log(prevScore)
			} else if ('question' in recvData) {
				selectedAnswer = null;

				prevScore = structuredClone(currentScore);
				currentScore = null;
				question = recvData;
				prevQuestion = structuredClone(question);	

				currentQuestionTimeLeftSeconds = CURRENT_QUESTION_TIME 

			} else if ('user_count' in recvData) {
				infoCounts = recvData;
			} else {
				question = null;
			}
		}
	};

	const decreaseTime = () => {
		if (state == "question") {
			currentQuestionTimeLeftSeconds -= 1
			if (currentQuestionTimeLeftSeconds == 0) {
				onScoreboard()
			}
		}
	};
	setInterval(decreaseTime, 1000)

	const roomSocket = new WebSocket(import.meta.env.VITE_ROOM_WEBSOCKET_URL);

	onDestroy(() => {
		roomSocket.close();
	});
	roomSocket.addEventListener('message', recvMessage);

	const sendMessage = (msg: string) => {
		if (!roomSocket || roomSocket.readyState !== WebSocket.OPEN) return;
		roomSocket.send(msg);
	};

	setInterval(async () => {
		if (isAdmin) sendMessage("u")
	}, 200);

	const enterRoom = () => {
		enteredRoom = true;
		state = 'question';
		sendMessage('b');
	};

	const onScoreboard = () => {
		sendMessage('r');

		state = 'scoreboard';
		sendMessage('s');
	};

	const onAnswerSelection = (msg: number) => {
		if (selectedAnswer) {
			return;
		}
		selectedAnswer = msg;
		sendMessage(`a${msg}`);
	};
</script>

{#if state == 'starting'}
	<div class="flex h-full items-center justify-center gap-4">
		<SButton class="bg-secondary-300" on:click={enterRoom}>Beitreten</SButton>

		{#if isAdmin}
			<!-- is admin -->
			<SButton
				class="bg-primary-300"
				on:click={() => {
					sendMessage(`h${jwtToken}`);
					waitingForQuestions = true;
					state = 'firstQuestion';
				}}>Admin</SButton
			>
		{/if}
	</div>
{/if}

<div class="h-[90%]">
	{#if isAdmin && !enteredRoom}
		<div class="flex justify-between">
			<div></div>
			<!-- is admin -->
			{#if state == 'question'}
				<SButton on:click={onScoreboard}>Weiter</SButton>
			{:else if state == 'scoreboard'}
				<SButton
					on:click={() => {
						sendMessage('n');
						state = 'question';
					}}>Nächste Frage</SButton
				>
			{:else if state == 'firstQuestion'}
				<SButton
					on:click={() => {
						sendMessage('n');
						state = 'question';
					}}>Erste Frage</SButton
				>
			{/if}
		</div>
	{/if}

	{#if isAdmin}
		{#if question}
            <h1 class="text-7xl font-bold text-center">
                {question.question}
            </h1>
			<div class="content mt-5">
				<div class="squares-container">
					<div
						class="square bg-primary-400 flex justify-center items-center text-center text-xl"
					>
						{question.answer1}
					</div>
					<div
						class="square bg-secondary-400 flex justify-center items-center text-center text-xl"
					>
						{question.answer2}
					</div>
					<div
						class="square bg-tertiary-500 flex justify-center items-center text-center text-xl"
					>
						{question.answer3}
					</div>
					<div
						class="square bg-surface-500 flex justify-center items-center text-center text-xl text-white"
					>
						{question.answer4}
					</div>
				</div>
			</div>
		{:else if state == 'firstQuestion'}
			<div class="flex h-[95%] flex-col items-center justify-center gap-4">
                <h2 class="text-7xl font-bold">
                    {infoCounts?.user_count ?? 0} Teilnehmer
                </h2>
            </div>
		{:else}
			<div class="flex h-[95%] flex-col items-center justify-center gap-4">
				<span class="text-3xl">"<span class="font-bold">{#if currentScore?.correct_answer == 1}
					{prevQuestion?.answer1}
				{/if}

				{#if currentScore?.correct_answer == 2}
					{prevQuestion?.answer2}
				{/if}

				{#if currentScore?.correct_answer == 3}
					{prevQuestion?.answer3}
				{/if}

				{#if currentScore?.correct_answer == 4}
					{prevQuestion?.answer4}
				{/if}</span>" 
				wäre die richtige Antwort.</span>
                <span class="text-4xl font-bold">
					Scoreboard
				</span>
                <table class="table table-hover w-[70%]">
                    <thead>
                        <tr>
                            <th>Teilnehmer</th>
                            <th>Punkte</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each (currentScoreboard ?? []).slice(0, 5) as player}
                            <tr>
                                <td>{player.name}/{player.id}</td>
                                <td>{player.score}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
		{/if}
	{:else if question}
		<div class="content">
			<div class="squares-container">
				<div
					on:click={() => onAnswerSelection(1)}
					on:keydown
					role="button"
					tabindex="0"
					class="square {selectedAnswer == 1
						? 'opacity-55'
						: ''} bg-primary-400 flex justify-center items-center text-center text-xl"
				>
					{question.answer1}
				</div>
				<div
					on:click={() => onAnswerSelection(2)}
					on:keydown
					role="button"
					tabindex="0"
					class="square {selectedAnswer == 2
						? 'opacity-55'
						: ''} bg-secondary-400 flex justify-center items-center text-center text-xl"
				>
					{question.answer2}
				</div>
				<div
					on:click={() => onAnswerSelection(3)}
					on:keydown
					role="button"
					tabindex="0"
					class="square {selectedAnswer == 3
						? 'opacity-55'
						: ''} bg-tertiary-500 flex justify-center items-center text-center text-xl"
				>
					{question.answer3}
				</div>
				<div
					on:click={() => onAnswerSelection(4)}
					on:keydown
					role="button"
					tabindex="0"
					class="square {selectedAnswer == 4
						? 'opacity-55'
						: ''} bg-surface-500 flex justify-center items-center text-center text-xl text-white"
				>
					{question.answer4}
				</div>
			</div>
		</div>
	{:else if currentScore}
		<div class="flex h-[95%] flex-col items-center justify-center gap-2">

			<p class="{currentScore.correct_answer == selectedAnswer ? "bg-green-700" : "bg-red-700" }  rounded-lg text-white text-center py-4 w-96">
				<span class="font-bold text-lg">
					{#if currentScore.correct_answer == selectedAnswer}
						RICHTIG!
					{:else}
						FALSCH!
					{/if}
				</span>
			</p>

			<p class="bg-surface-500 rounded-lg text-white text-center py-4 w-96">
				<span class="font-bold text-lg">
					+{(currentScore?.score ?? 0) - (prevScore?.score ?? 0)}
				</span>
			</p>

			<p class="bg-surface-500 rounded-lg text-white text-center py-4 w-96">
				<span class="font-bold text-lg">
					{currentScore?.score ?? 0} Punkte
				</span>
			</p>
			<p class="bg-surface-500 rounded-lg text-white text-center py-4 w-96">
				<span class="font-bold text-lg">
					{currentScore?.place}. Platz
				</span>
			</p>
		</div>

	{:else}
		<div class="flex h-[95%] flex-col items-center justify-center gap-2">
			<span class="text-5xl font-bold">Warten auf Fragen..</span>
		</div>
	{/if}

	<section>
		<hr />
		{#if isAdmin && state == "question"}
            <div class="flex justify-between">
                <h2 class="text-2xl font-bold">
                    {infoCounts?.answer_count} 
                    {#if infoCounts?.answer_count == 1}
                        Antwort
                    {:else}
                        Antworten
                    {/if}
                </h2>

				<div class="text-xl">
					<span class="font-bold">{currentQuestionTimeLeftSeconds}</span> Sekunden übrig
				</div>
                <h2 class="text-2xl font-bold">{infoCounts?.user_count} Teilnehmer</h2>
            </div>
        {:else if userName}
			{userName}/{userId}
		{/if}
	</section>
</div>

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
		/* .squares-container {
    grid-template-columns: 1fr;
  } */

		.square {
			height: 290px;
			width: 100%;
		}

		.content {
			gap: 10px;
		}
	}
</style>
