import type { Delegate, VoteResult } from '$lib/types';
import type { AutocompleteOption } from './types';

export function convertDelegatesToAutocompleteOptions(
	delegates: Delegate[],
	otherKeywords: string[] = [],
	voteResult: VoteResult | null = null
): AutocompleteOption<string>[] {
	return delegates.map((delegate) => {
		let genderIdents: string[] = [];
		if (delegate.gender == 'm') {
			genderIdents = ['männlich', 'mann', 'abgeordneter'];
		} else if (delegate.gender == 'f') {
			genderIdents = ['weiblich', 'frau', 'abgeordnete'];
		}

		let yes_or_no_vote = '';
		if (voteResult && voteResult.named_votes) {
			voteResult.named_votes.named_votes.forEach((nv) => {
				if (nv.delegate_id == delegate.id) {
					yes_or_no_vote = nv.infavor ? 'ja' : 'nein';
				}
			});
		}

		let genderIdentsString = genderIdents.join(', ');
		let divisionsString = delegate.divisions?.join(', ');
		let otherKeywordsString = otherKeywords.join(', ');

		const rightLabel = delegate.party ? delegate.party : "OK";

		return {
			right_label: rightLabel,
			label: delegate.name,
			value: delegate.name,
			keywords: `${delegate.id}, ${delegate.party}, ${delegate.constituency}, ${genderIdentsString}, ${delegate.birthdate}, ${delegate.active_since}, ${divisionsString}, ${otherKeywordsString}, ${yes_or_no_vote}`,
			meta: delegate
		};
	});
}

export function delegateFilterOptions(
	_options: AutocompleteOption<string>[],
	_inputValue: string
): AutocompleteOption<string>[] {
	return _options.filter((option, i) => {
		// if (i >= 16) return null;
		let values = _inputValue.split(' ');
		const optionFormatted = `${option.value.toLowerCase().trim()} ${option.keywords?.toLowerCase().trim()}`;
		for (let idx = 0; idx < values.length; idx++) {
			if (!optionFormatted.includes(values[idx])) {
				return null;
			}
		}
		return option;
	});
}
