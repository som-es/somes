<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import { getQuizzes } from '$lib/api/authed';
	import { jwtQuizStore, jwtStore } from '$lib/caching/stores/stores';
	import ReactiveGenericBarChart from '$lib/components/GeneralCharts/ReactiveGenericBarChart.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import {
		getUserFromJwt,
		type BasicUserInfo,
		type InfoCounts,
		type Quiz,
		type QuizQuestion,
		type ScoreInfo,
		type Scorer
	} from '$lib/types';
	import {
		ListBox,
		ListBoxItem,
		popup,
		setModeCurrent,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import { onDestroy, onMount } from 'svelte';
	import { get } from 'svelte/store';

	let enteredRoom = false;
	let recvUserCount = true;
	let userName: string | null = null;
	let userId: string | null = null;
	let waitingForQuestions = false;
	let prevQuestion: QuizQuestion | null = null;
	let question: QuizQuestion | null = null;
	let allQuizzes: Quiz[] = [];

	setModeCurrent(true);

	let currentScoreboard: Scorer[] | null = null;

	let prevScore: ScoreInfo | null = null;
	let currentScore: ScoreInfo | null = null;
	let infoCounts: InfoCounts | null = null;

	let state = 'starting';
	let isAdmin = false;

	let jwtToken: string | null;
	let jwtQuizToken: string | null = get(jwtQuizStore);
	let selectedAnswer: number | null = null;

	const CURRENT_QUESTION_TIME = 20;

	let currentQuestionTimeLeftSeconds = CURRENT_QUESTION_TIME;

	onMount(async () => {
		jwtToken = get(jwtStore);

		if (jwtToken) {
			const user = getUserFromJwt(jwtToken);
			isAdmin = user.is_admin;
			if (isAdmin) {
				allQuizzes = errorToNull(await getQuizzes()) ?? [];
			}
		}

		if (jwtQuizToken && !isAdmin) {
			state = 'tryToken';
		}
	});

	const recvMessage = (event: MessageEvent) => {
		console.log(event.data);
		if (state == 'tryToken' && jwtQuizToken) {
			const data = event.data as string;
			if (data == 'ok') {
				enteredRoom == true;
				waitingForQuestions = true;
				state = 'question';
				console.log('OK');

				const user = getUserFromJwt(jwtQuizToken);
				userName = user.sub;
				userId = user.id.toString();
			} else {
				jwtQuizStore.set(null);
				userName = null;
				userId = null;
				state = 'starting';
				enteredRoom == false;
				console.log('HIHIHI');
			}
			return;
		}
		if (enteredRoom && !waitingForQuestions) {
			const data = event.data as string;
			userName = data.slice(0, data.indexOf(';'));
			const jwt = data.slice(data.indexOf(';') + 1);
			jwtQuizStore.set(jwt);

			const user = getUserFromJwt(jwt);
			userId = user.id.toString();

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
				console.log(prevScore);
			} else if ('question' in recvData) {
				selectedAnswer = null;

				prevScore = structuredClone(currentScore);
				currentScore = null;
				question = recvData;
				prevQuestion = structuredClone(question);

				currentQuestionTimeLeftSeconds = CURRENT_QUESTION_TIME;
			} else if ('user_count' in recvData) {
				infoCounts = recvData;
			} else {
				question = null;
			}
		}
	};

	const decreaseTime = () => {
		if (state == 'question' && isAdmin) {
			currentQuestionTimeLeftSeconds -= 1;
			if (currentQuestionTimeLeftSeconds == 0) {
				onScoreboard();
			}
		}
	};
	setInterval(decreaseTime, 1000);

	const roomSocket = new WebSocket(import.meta.env.VITE_ROOM_WEBSOCKET_URL);

	onDestroy(() => {
		roomSocket.close();
	});
	roomSocket.addEventListener('message', recvMessage);
	roomSocket.addEventListener('open', () => {
		if (jwtQuizToken && !isAdmin) {
			state = 'tryToken';
			const tokenPayload = `l${jwtQuizToken}`;
			console.log(tokenPayload);
			sendMessage(tokenPayload);
			// const user = getUserFromJwt(jwtQuizToken);
		}
	});

	const sendMessage = (msg: string) => {
		if (!roomSocket || roomSocket.readyState !== WebSocket.OPEN) return;
		roomSocket.send(msg);
	};

	setInterval(async () => {
		if (isAdmin) sendMessage('u');
	}, 200);

	const enterRoom = () => {
		enteredRoom = true;

		state = 'question';
		sendMessage('b');

		// sendMessage(`h${jwtQuizToken}`);
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

	const popupQuizSelection: PopupSettings = {
		event: 'click',
		target: 'popupQuizSelection',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	let inputQuizId: number = 4;
</script>

<div class="card w-48 shadow-xl py-2" data-popup="popupQuizSelection">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		{#each allQuizzes as quiz}
			<ListBoxItem bind:group={inputQuizId} name="inputQuizId" value={quiz.id}
				>{quiz.title}</ListBoxItem
			>
		{/each}
	</ListBox>
</div>

{#if state == 'starting'}
	<div class="flex h-full items-center justify-center gap-4">
		<SButton class="bg-secondary-300" on:click={enterRoom}>Beitreten</SButton>

		{#if isAdmin}
			<!-- is admin -->
			<SButton
				class="bg-primary-300"
				on:click={() => {
					sendMessage(`h${jwtToken};${inputQuizId}`);
					waitingForQuestions = true;
					state = 'firstQuestion';
				}}>Admin</SButton
			>

			<div class="flex flex-wrap gap-6">
				<div>
					<h1 class="text-2xl font-bold">Quiz</h1>
					<button
						class="btn variant-filled-secondary w-48 justify-between"
						use:popup={popupQuizSelection}
					>
						<span class="capitalize">Quiz</span>
						<span>↓</span>
					</button>
				</div>
			</div>
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
					<div class="square bg-primary-400 flex justify-center items-center text-center text-xl">
						{question.answer1}
					</div>
					<div class="square bg-secondary-400 flex justify-center items-center text-center text-xl">
						{question.answer2}
					</div>
					<div class="square bg-tertiary-500 flex justify-center items-center text-center text-xl">
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
				<span class="text-3xl"
					>"<span class="font-bold">
						{#if currentScore?.correct_answer == 1}
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
						{/if}</span
					>" wäre die richtige Antwort.</span
				>

				<div class="flex flex-wrap gap-5 min-w-full">
					<div style=" flex-basis: 48%">

					<ReactiveGenericBarChart
						legend
						chartData={[
							{
								label: prevQuestion?.answer1 ?? '',
								color: '#95a7bd',
								data: infoCounts?.answer_count[1] ?? 0
							},
							{
								label: prevQuestion?.answer2 ?? '',
								color: '#e4a69a',
								data: infoCounts?.answer_count[2] ?? 0
							},
							{
								label: prevQuestion?.answer3 ?? '',
								color: '#F8DFCA',
								data: infoCounts?.answer_count[3] ?? 0
							},
							{
								label: prevQuestion?.answer4 ?? '',
								color: '#24283E',
								data: infoCounts?.answer_count[4] ?? 0
							}
						]}
						title=""
						horizontalBars={true}
					/>
					</div>

					<div style=" flex-basis: 48%">
						<span class="text-4xl font-bold"> Scoreboard </span>
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
				</div>
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
			<p
				class="{currentScore.correct_answer == selectedAnswer
					? 'bg-green-700'
					: 'bg-red-700'}  rounded-lg text-white text-center py-4 w-96"
			>
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
			{#if userName}
				Du spielst als <span class="text-xl font-bold">{userName}</span>
			{/if}
		</div>
	{/if}

	<section>
		<hr />
		{#if isAdmin && state == 'question'}
			<div class="flex justify-between">
				<h2 class="text-2xl font-bold">
					{infoCounts?.answer_count[0]}
					{#if infoCounts?.answer_count[0] == 1}
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
