export interface DelegateSplit {
	nr: Delegate[];
	gov: Delegate[];
	all: Delegate[];
}

export interface DelegateData {
	name: string;
	party: string;
	data: number;
}

export interface Delegate {
	id: number;
	name: string;
	party: string;
	current_party: string;
	image_url: string;
	constituency: string;
	council: string;
	seat_row: number | null;
	seat_col: number | null;
	gender: string | null;
	is_active: boolean | null;
	birthdate: string;
	active_since: Date;
	primary_mandate: string;
	divisions: string[] | null;
	active_mandates: string[] | null;
}

export interface DbMinistrialProposalQuery {
	id: number;
	ityp: string;
	gp: string;
	inr: number;
	emphasis: string | null;
	title: string;
	description: string;
	created_at: string;
	updated_at: string | null;
	due_to: string;
	ressort: string | null;
	ressort_shortform: string | null;
	legis_init_gp: string | null;
	legis_init_inr: number | null;
	legis_init_ityp: string | null;
}

export interface GovProposalDelegate {
	delegate: Delegate,
	gov_proposal: GovProposal,
}

export interface GovProposal {
	ministrial_proposal: DbMinistrialProposalQuery;
	vote_result: VoteResult | null;
	topics: Topic[];
}

export interface LegislativeInitiative {
	id: string;
	ityp: string;
	gp: string;
	inr: number;
	title: string;
	description: string;
	emphasis: string | null;
	accepted: string | null;
	created_at: Date;
	appeared_at: Date | null;
	updated_at: Date | null;
	requires_simple_majority: boolean | null;
	voted_by_name: boolean | null;
	is_emphasis_ai_generated: boolean | null;
}

export interface Vote {
	party: string;
	fraction: number;
	infavor: boolean;
	legislative_initiatives_id: number;
}

export interface Speech {
	delegate_id: number;
	infavor: boolean | null;
	opinion: string | null;
	document_url: string | null;
	legislative_initiatives_id: number;
}

export interface SpeechesWithMaxPage {
	speeches: Speech[];
	entry_count: number;
	max_page: number;
}

export interface Document {
	title: string | null;
	document_url: string;
	document_type: string;
}

export interface Topic {
	topic: string;
}

export interface UniqueTopic {
	id: number;
	topic: string;
}

export interface VoteResult {
	legislative_initiative: LegislativeInitiative;
	votes: Vote[];
	speeches: Speech[];
	topics: Topic[];
	named_votes: NamedVotes | null;
	documents: Document[];
	absences: number[];
}

export interface VoteResultsWithMaxPage {
	vote_results: VoteResult[];
	entry_count: number;
	max_page: number;
}

export interface WaloQuestion {
	id: number;
	question_statement: string | null;
	new_keywords_topics: string | null;
	spoe_justification: string | null;
	gruene_justification: string | null;
	neos_justification: string | null;
	fpoe_justification: string | null;
	oevp_justification: string | null;
	somes_link: string | null;
	law_link: string | null;
	erklaerbaer: string | null;
}

export interface SpeakerByHours {
	name: string;
	image_url: string | null;
	party: string | null;
	hours_spoken: number;
}

export interface DelegateByCallToOrders {
	name: string;
	image_url: string | null;
	party: string | null;
	call_to_order_amount: number;
}

export interface CallToOrdersPerPartyDelegate {
	party: string;
	call_to_order_amount: number;
	delegate_amount: number;
	ratio: number;
}

export interface PoliticalPosition {
    delegate_id: number;
    is_left: number;
    is_not_left: number;
    is_liberal: number;
    is_not_liberal: number;
    neutral_count: number;
}

export interface Party {
	name: string;
	color: string;
	fraction: number;
	code: string;
}

export interface HasError {
	error: string;
}

export interface JWTInfo {
	access_token: string;
}

export function jwtDecode(t: string) {
	return {
		raw: t,
		header: JSON.parse(window.atob(t.split('.')[0])),
		payload: JSON.parse(window.atob(t.split('.')[1]))
	};
}

export function getUserFromJwt(t: string): BasicUserInfo {
	console.log(jwtDecode(t).payload);
	return jwtDecode(t).payload;
}

export interface BasicUserInfo {
	id: number;
	sub: string;
	company: string;
	exp: number;
}

export interface SignUpError {
	missing_username: boolean;
	missing_password: boolean;
	missing_email: boolean;
	username_taken: boolean;
	email_taken: boolean;
	invalid_email: boolean;
	insufficient_password: boolean;
	is_errorneous: boolean;
}

export interface Question {
	question_id: number;
	issuer_id: number; // user?
	created_on: Date;
	delegate_id: number;
	title: string;
	body: string;
	response: string | null;
	responded_on: Date | null;
	editable: boolean;
	last_edited_on: Date | null;
	visible: boolean;
	likes: number;
	dislikes: number;
}

export interface QuestionFilter {
	page: number;
	filter_text: string | null;
	filter_delegate: number | null;
	filter_party: number | null;
	filter_date_range: DateRange | null;
	filter_topics: string[] | null; // maybe
}

export interface DateRange {
	start: Date;
	end: Date;
}

export interface Mandate {
	start_date: Date;
	end_date: Date | null;
	name: string;
}

export interface GeneralDelegateInfo {
	interests: InterestShare[];
	delegate_qa: DelegateQA[];
	mandates: Mandate[];
	political_position: PoliticalPosition | null;
}

export interface DelegateQA {
	question: string;
	answer: string;
}

export interface InterestShare {
	topic: string;
	total_share: number;
	self_share: number;
}

export interface LegisPeriod {
	gp: string;
	start_date: Date;
}

export interface VoteResultFilter {
	is_named_vote: boolean | null;
	accepted: string | null;
	simple_majority: boolean | null;
	legis_period: string | null;
}

export interface NamedVote {
	id: number;
	infavor: boolean | null;
	was_absent: boolean | null;
	lev: number;
	similiarity_score: number;
	searched_with: string | null;
	matched_with: string;
	delegate_id: number;
	named_vote_info_id: number;
	manually_matched: boolean | null;
}

export interface NamedVotes {
	named_vote_info: NamedVoteInfo;
	named_votes: NamedVote[];
}

export interface NamedVoteInfo {
	id: number;
	legis_init_id: number;
	pro_count: number;
	contra_count: number;
	given_vote_sum: number;
	invalid_count: number;
}

export interface LoginResponseError {
	missing_username: false;
	missing_password: false;
	missing_email: false;
	username_taken: false;
	email_taken: false;
	invalid_email: true;
	insufficient_password: false;
	invalid_otp: false;
	is_erroneous: true;
}

export function areDeeplyEqual(param1: unknown, param2: unknown) {
	// check strict equality
	if (param1 === param2) return true;
	// check if props are not objects
	if (!(param1 instanceof Object) || !(param2 instanceof Object)) return false;

	// object keys
	const keys1 = Object.keys(param1);
	const keys2 = Object.keys(param2);
	// check if number of keys are the same
	if (keys1.length !== keys2.length) return false;
	// Iterate over the keys and compare the values recursively
	for (const key of keys1) {
		const value1 = (param1 as Record<string, unknown>)[key];
		const value2 = (param2 as Record<string, unknown>)[key];
		if (!areDeeplyEqual(value1, value2)) return false;
	}
	return true;
}
