import { resolve } from "$app/paths";
import { page } from "$app/state";
import type { VoteResultFilter } from "$lib/types";

export function convertVoteResultFilterToUrl(filter: VoteResultFilter | null, searchValue: string, currentUrl: URL | undefined, isFinished: boolean = true): URL {
    
    const nextUrl = currentUrl ? currentUrl : new URL(isFinished ? resolve("/history/votes") : resolve("/history/unfinished_votes"), page.url.origin);
     
    let paramPage = nextUrl.searchParams.get('page');
    if (paramPage == null) {
        paramPage = '1';
    }

    nextUrl.search = '';
    
    if (paramPage) nextUrl.searchParams.set('page', paramPage);
    if (filter === null) {
        return nextUrl 
    }
    if (filter.is_named_vote !== null) {
        nextUrl.searchParams.set(
            'legislative_initiative[voted_by_name][eq]',
            filter.is_named_vote.toString()
        );
    }
    if (filter.accepted !== null) {
        nextUrl.searchParams.set('legislative_initiative[accepted][eq]', filter.accepted);
    }
    if (filter.vote_type.length > 0) {
        nextUrl.searchParams.set('legislative_initiative[voting][in][0]', filter.vote_type[0]);
    }

    if (filter.gps.length > 0) {
        nextUrl.searchParams.set('legislative_initiative[gp][in][0]', filter.gps[0]);
    }

    if (filter.simple_majority !== null) {
        nextUrl.searchParams.set(
            'legislative_initiative[requires_simple_majority][eq]',
            filter.simple_majority.toString()
        );
    }

    filter.party_votes?.forEach((partyVotes, i) => {
        nextUrl.searchParams.set(`party_votes[${i}][infavor]`, partyVotes.infavor.toString());
        nextUrl.searchParams.set(`party_votes[${i}][party]`, partyVotes.party);
    });

    nextUrl.searchParams.set('search', searchValue);

    filter.topics?.forEach((topic, i) => {
        nextUrl.searchParams.set(`eurovoc_topics[${i}][topic][cn]`, topic);
    });

    return nextUrl;
}