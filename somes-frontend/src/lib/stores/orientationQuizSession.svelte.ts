export const orientationQuizSession = $state({
	quizType: null as 'short' | 'long' | null,
	step: 'start' as 'start' | 'quiz' | 'result',
	currentIndex: 0,
	answers: {} as Record<number, number | null>,
	strongRefMode: true
});
