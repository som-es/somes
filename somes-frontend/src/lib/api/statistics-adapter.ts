import type { StatisticsData, DelegateStatistics, CategoryStatistics } from '$lib/types';

// Call to Orders adapters
export function mapCallToOrdersDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate' as const,
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.normalized_calls_to_order || item.total_order_calls,
		metadata: {
			total_order_calls: item.total_order_calls,
			total_sessions_attended: item.total_sessions_attended,
			normalized_calls_to_order: item.normalized_calls_to_order
		}
	}));
}

export function mapCallToOrdersCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.normalized_calls_to_order || item.total_order_calls,
		metadata: {
			total_order_calls: item.total_order_calls,
			total_sessions_attended: item.total_sessions_attended,
			normalized_calls_to_order: item.normalized_calls_to_order
		}
	}));
}

// Absences adapters
export function mapAbsencesDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate' as const,
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.normalized_absences || item.total_absences,
		metadata: {
			total_absences: item.total_absences,
			total_sessions: item.total_sessions,
			normalized_absences: item.normalized_absences
		}
	}));
}

export function mapAbsencesCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.normalized_absences || item.total_absences,
		metadata: {
			total_absences: item.total_absences,
			total_sessions: item.total_sessions,
			normalized_absences: item.normalized_absences
		}
	}));
}

// Activity adapters
export function mapActivityDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate' as const,
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.activity_score,
		metadata: {
			activity_score: item.activity_score,
			total_proposals: item.total_proposals,
			mandate_duration_days: item.mandate_duration_days
		}
	}));
}

export function mapActivityCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.activity_score,
		metadata: {
			activity_score: item.activity_score,
			total_proposals: item.total_proposals,
			delegate_count: item.delegate_count
		}
	}));
}

// Age adapters
export function mapAgeDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate' as const,
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.age,
		metadata: {
			age: item.age
		}
	}));
}

export function mapAgeCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.average_age,
		metadata: {
			average_age: item.average_age,
			delegate_count: item.delegate_count,
			min_age: item.min_age,
			max_age: item.max_age
		}
	}));
}

// Complexity adapters
export function mapComplexityDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.complexity_score,
		metadata: {
			complexity_score: item.complexity_score,
			total_proposals: item.total_proposals
		}
	}));
}

export function mapComplexityCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.average_complexity,
		metadata: {
			average_complexity: item.average_complexity,
			total_proposals: item.total_proposals,
			delegate_count: item.delegate_count
		}
	}));
}

// Speech Time adapters
export function mapSpeechtimeDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.average_speech_time,
		metadata: {
			total_speeches: item.total_speeches,
			total_speech_time: item.total_speech_time,
			average_speech_time: item.average_speech_time
		}
	}));
}

export function mapSpeechtimeCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.average_speech_time,
		metadata: {
			total_speeches: item.total_speeches,
			total_speech_time: item.total_speech_time,
			average_speech_time: item.average_speech_time
		}
	}));
}

// Total Speeches adapters
export function mapTotalSpeechesDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.total_speeches,
		metadata: {
			total_speeches: item.total_speeches,
			total_speech_time: item.total_speech_time,
			average_speech_time: item.average_speech_time
		}
	}));
}

export function mapTotalSpeechesCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category' as const,
		category: item.category,
		value: item.total_speeches,
		metadata: {
			total_speeches: item.total_speeches,
			total_speech_time: item.total_speech_time,
			average_speech_time: item.average_speech_time
		}
	}));
}

// Division Accuracy adapters
export function mapDivisionAccuracyDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.accuracy_score,
		metadata: {
			accuracy_score: item.accuracy_score,
			total_votes: item.total_votes
		}
	}));
}

export function mapDivisionAccuracyCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category',
		category: item.category,
		value: item.average_accuracy,
		metadata: {
			average_accuracy: item.average_accuracy,
			total_votes: item.total_votes,
			delegate_count: item.delegate_count
		}
	}));
}

// Political Orientation adapters (is_left)
export function mapIsLeftDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.is_left,
		metadata: {
			is_left: item.is_left,
			is_not_left: item.is_not_left,
			neutral_count: item.neutral_count
		}
	}));
}

export function mapIsLeftCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category',
		category: item.category,
		value: item.average_is_left || item.total_is_left,
		metadata: {
			average_is_left: item.average_is_left,
			total_is_left: item.total_is_left,
			delegate_count: item.delegate_count
		}
	}));
}

// Political Orientation adapters (is_liberal)
export function mapIsLiberalDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.is_liberal,
		metadata: {
			is_liberal: item.is_liberal,
			is_not_liberal: item.is_not_liberal,
			neutral_count: item.neutral_count
		}
	}));
}

export function mapIsLiberalCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category',
		category: item.category,
		value: item.average_is_liberal || item.total_is_liberal,
		metadata: {
			average_is_liberal: item.average_is_liberal,
			total_is_liberal: item.total_is_liberal,
			delegate_count: item.delegate_count
		}
	}));
}

// Speeches adapters (speechtime)
export function mapSpeechTimeDelegate(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'delegate',
		name: item.delegate_name,
		party: item.delegate_party,
		value: item.total_speech_time || item.speechtime,
		metadata: {
			total_speech_time: item.total_speech_time || item.speechtime,
			total_speeches: item.total_speeches
		}
	}));
}

export function mapSpeechTimeCategory(data: any[]): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category',
		category: item.category,
		value: item.average_speech_time || item.total_speech_time,
		metadata: {
			average_speech_time: item.average_speech_time,
			total_speech_time: item.total_speech_time,
			delegate_count: item.delegate_count
		}
	}));
}


// Generic adapter for simple category-based statistics
export function mapGenericCategory(data: any[], valueField: string = 'value'): StatisticsData[] {
	return data.map((item: any) => ({
		type: 'category',
		category: item.category,
		value: item[valueField],
		metadata: item
	}));
}
