import type { StatisticsData } from '$lib/types';

// Generic mapper helper function
function mapData(
	data: any[],
	config: {
		type: 'delegate' | 'category';
		labelField: string;
		valueField: string;
		partyField?: string;
		metadataFields?: string[];
	}
): StatisticsData[] {
	return data.map((item) => ({
		type: config.type,
		label: item[config.labelField],
		value: item[config.valueField],
		party: config.partyField ? item[config.partyField] : undefined,
		metadata: config.metadataFields
			? Object.fromEntries(config.metadataFields.map((f) => [f, item[f]]))
			: item
	}));
}

// Call to Orders adapters
export function mapCallToOrdersDelegate(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: normalized ? 'normalized_calls_to_order' : 'total_order_calls',
		partyField: 'delegate_party',
		metadataFields: ['total_order_calls', 'total_sessions_attended', 'normalized_calls_to_order']
	});
}

export function mapCallToOrdersCategory(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: normalized ? 'normalized_calls_to_order' : 'total_order_calls',
		metadataFields: ['total_order_calls', 'total_sessions_attended', 'normalized_calls_to_order']
	});
}

// Absences adapters
export function mapAbsencesDelegate(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: normalized ? 'normalized_absences' : 'total_absences',
		partyField: 'delegate_party',
		metadataFields: ['total_absences', 'total_sessions', 'normalized_absences']
	});
}

export function mapAbsencesCategory(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: normalized ? 'normalized_absences' : 'total_absences',
		metadataFields: ['total_absences', 'total_sessions', 'normalized_absences']
	});
}

export function mapActivityDelegate(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: normalized ? 'activity_score' : 'raw_activity_score',
		partyField: 'delegate_party',
		metadataFields: ['activity_score', 'raw_activity_score', 'total_proposals', 'session_count']
	});
}

export function mapActivityCategory(data: any[], normalized: boolean = true): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: normalized ? 'activity_score' : 'raw_activity_score',
		metadataFields: ['activity_score', 'raw_activity_score', 'total_proposals', 'delegate_count']
	});
}

export function mapAgeDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'age',
		partyField: 'delegate_party',
		metadataFields: ['age']
	});
}

export function mapAgeCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_age',
		metadataFields: ['average_age', 'delegate_count', 'min_age', 'max_age']
	});
}

export function mapComplexityDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'complexity_score',
		partyField: 'delegate_party',
		metadataFields: ['complexity_score', 'total_proposals']
	});
}

export function mapComplexityCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_complexity',
		metadataFields: ['average_complexity', 'total_proposals', 'delegate_count']
	});
}

export function mapSpeechTimeDelegate(data: any[], normalized: boolean = false): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: normalized ? 'average_speech_time' : 'total_speech_time',
		partyField: 'delegate_party',
		metadataFields: ['total_speeches', 'total_speech_time', 'average_speech_time']
	});
}

export function mapSpeechtimeCategory(data: any[], normalized: boolean = false): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: normalized ? 'average_speech_time' : 'total_speech_time',
		metadataFields: ['total_speeches', 'total_speech_time', 'average_speech_time']
	});
}

export function mapTotalSpeechesDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'total_speeches',
		partyField: 'delegate_party',
		metadataFields: ['total_speeches', 'total_speech_time', 'average_speech_time']
	});
}

export function mapTotalSpeechesCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'total_speeches',
		metadataFields: ['total_speeches', 'total_speech_time', 'average_speech_time']
	});
}

export function mapDivisionAccuracyDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'accuracy_score',
		partyField: 'delegate_party',
		metadataFields: ['accuracy_score', 'total_votes']
	});
}

export function mapDivisionAccuracyCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_accuracy',
		metadataFields: ['average_accuracy', 'total_votes', 'delegate_count']
	});
}

export function mapIsLeftDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'is_left',
		partyField: 'delegate_party',
		metadataFields: ['is_left', 'is_not_left', 'neutral_count']
	});
}

export function mapIsLeftCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_is_left',
		metadataFields: ['average_is_left', 'total_is_left', 'delegate_count']
	});
}

// Political Orientation adapters (is_liberal)
export function mapIsLiberalDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'is_liberal',
		partyField: 'delegate_party',
		metadataFields: ['is_liberal', 'is_not_liberal', 'neutral_count']
	});
}

export function mapIsLiberalCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_is_liberal',
		metadataFields: ['average_is_liberal', 'total_is_liberal', 'delegate_count']
	});
}

export function mapOrientationDelegate(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'delegate',
		labelField: 'delegate_name',
		valueField: 'orientation_score',
		partyField: 'delegate_party',
		metadataFields: ['orientation_score', 'total_votes']
	});
}

export function mapOrientationCategory(data: any[]): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: 'average_orientation',
		metadataFields: ['average_orientation', 'total_votes', 'delegate_count']
	});
}

export function mapPoliticalSpectrumDelegate(data: any[]): StatisticsData[] {
	return data.map((item) => ({
		type: 'delegate',
		label: item.delegate_name,
		value: item.spectrum_magnitude,
		party: item.delegate_party,
		metadata: {
			left_right_score: item.left_right_score,
			liberal_authoritarian_score: item.liberal_authoritarian_score,
			spectrum_magnitude: item.spectrum_magnitude,
			total_votes: item.total_votes
		}
	}));
}

export function mapPoliticalSpectrumCategory(data: any[]): StatisticsData[] {
	return data.map((item) => ({
		type: 'category',
		label: item.category,
		value: item.spectrum_magnitude,
		metadata: {
			left_right_score: item.average_left_right_score,
			liberal_authoritarian_score: item.average_liberal_authoritarian_score,
			spectrum_magnitude: item.spectrum_magnitude,
			total_votes: item.total_votes,
			delegate_count: item.delegate_count
		}
	}));
}

export function mapSpeechTimeCategory(data: any[], normalized: boolean = false): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: normalized ? 'average_speech_time' : 'total_speech_time',
		metadataFields: ['average_speech_time', 'total_speech_time', 'delegate_count']
	});
}

export function mapGenericCategory(data: any[], valueField: string = 'value'): StatisticsData[] {
	return mapData(data, {
		type: 'category',
		labelField: 'category',
		valueField: valueField
	});
}
