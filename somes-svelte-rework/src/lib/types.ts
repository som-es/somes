import type { Decree } from './components/Delegates/Decrees/types';

export interface DelegateSplit {
	nr: Delegate[];
	gov: Delegate[];
	all: Delegate[];
}

export interface DelegateData {
	name: string | null;
	party: string | null;
	data: number;
}

export interface ChartData {
	label: string;
	data: number;
	color: string;
}

export interface FullMandate {
	start_date: string | null;
	end_date: string | null;
	name: string | null;
	party: string | null;
	is_nr: boolean | null;
	is_gov_official: boolean | null;
	is_ministry: boolean | null;
	is_chancellor: boolean | null;
	function: string | null;
}

export interface Delegate {
	id: number;
	name: string;
	party: string;
	current_party: string;
	image_url: string | null;
	constituency: string;
	council: string;
	seat_row: number | null;
	seat_col: number | null;
	gender: string | null;
	is_active: boolean | null;
	birthdate: string;
	active_since: Date;
	divisions: string[] | null;
	mandates_at_time: FullMandate[] | null;
	active_mandates: FullMandate[] | null;
	mandates: FullMandate[] | null;
}

export interface DbMinistrialProposalQueryMeta {
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
	delegate: Delegate;
	gov_proposal: GovProposal;
}

export interface GovProposal {
	ministrial_proposal: DbMinistrialProposalQueryMeta;
	vote_result: VoteResult | null;
	topics: Topic[];
	eurovoc_topics: Topic[];
	other_keyword_topics: Topic[];
	ministerial_issuers: number[];
	documents: Document[];
}

export interface LegislativeInitiative {
	id: number;
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
	pre_declined_type: string | null;
	plenary_session_id: number | null;
	is_law: boolean;
	law_accepted: boolean | null;
	law_come_into_effect_date: string | null;
	law_expires_on_date: string | null;
	by_publication: boolean | null;
	voting: string | null;
	is_voteable_on: boolean;
	is_urgent: boolean;
}

export interface Vote {
	party: string;
	code: string | null;
	fraction: number;
	infavor: boolean;
	legislative_initiatives_id: number;
}

export interface Speech {
	delegate_id: number;
	infavor: boolean | null;
	opinion: string | null;
	document_url: string | null;
	about: string | null;
	duration_in_seconds: number | null;
	legis_init_id: number | null;
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

export interface DelegateFavo {
	delegate_id: number;
}

export interface LegisInitFavo {
	vote_result_id: number;
}

export interface MailSendInfo {
	send_new_vote_results_mails: boolean;
	send_new_delegate_activity_mails: boolean;
	send_new_ministrial_prop_mails: boolean;
	send_new_ministrial_prop_by_favo_mails: boolean;
}

export interface RelatedDelegate {
	delegate_id: number;
	text: string | null;
}

export interface Reference {
	gp: string;
	ityp: string;
	inr: number;
}
export interface VoteResult {
	legislative_initiative: LegislativeInitiative;
	votes: Vote[];
	speeches: Speech[];
	topics: Topic[];
	eurovoc_topics: Topic[];
	other_keyword_topics: Topic[];
	named_votes: NamedVotes | null;
	documents: Document[];
	absences: number[];
	issued_by_dels: RelatedDelegate[];
	referenced_by_others_ids: number[];
	references: Reference[];
}

export function createVoteResultPath(voteResult: VoteResult): string {
	return `/vote_result/${voteResult.legislative_initiative.gp}/${voteResult.legislative_initiative.ityp}/${voteResult.legislative_initiative.inr}`;
}

export interface VoteResultsWithMaxPage {
	vote_results: VoteResult[];
	entry_count: number;
	max_page: number;
}

export interface GovProposalsWithMaxPage {
	gov_proposals: GovProposalDelegate[];
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
	delegate_id: number | null;
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
	error_type: string;
	field: string;
	meta: any | null;
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
	is_admin: boolean;
}
export interface ExtendedUserInfo {
	id: number;
	email: string;
	is_email_hashed: boolean;
	is_admin: boolean;
}

export interface Quiz {
	id: number;
	description: string | null;
	title: string;
	questions: QuizQuestion[];
}

export interface QuizQuestion {
	question: string;
	answer1: string;
	answer2: string;
	answer3: string;
	answer4: string;
}

export interface Scorer {
	name: string;
	id: number;
	score: number;
}

export interface ScoreInfo {
	score: number;
	place: number;
	correct_answer: number;
}

export interface InfoCounts {
	user_count: number;
	answer_count: number[];
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

export interface Absence {
	date: Date;
	inr: number;
	gp: string;
	plenary_session_id: number;
	missed_legis_init_ids: number[];
}

export interface NamedVote {
	infavor: boolean | null;
	was_absent: boolean | null;
	legis_init_id: number;
	named_vote_info_id: number;
	date: Date;
}

export interface GeneralGovOfficialInfo {
	gov_proposals: GovProposal[];
	decrees: Decree[];
}

export interface StanceTopicInfluences {
	question: string;
	answer: string;
	stance_llm: string;
	topic_influences: StanceTopicScore[];
}

export interface GeneralDelegateInfo {
	interests: InterestShare[];
	detailed_interests: InterestShare[];
	delegate_qa: DelegateQA[];
	mandates: Mandate[];
	political_position: PoliticalPosition | null;
	absences: Absence[];
	named_votes: NamedVote[];
	stance_topic_influences: StanceTopicInfluences[];
	stance_topic_scores: StanceTopicScore[];
	left_right_stances: StanceTopicScore[];
}

export interface DelegateQA {
	question: string;
	answer: string;
}

export interface InterestShare {
	topic: string;
	total_share: number;
	occurences: number;
	self_share: number;
}

export interface LegisPeriod {
	gp: string;
	start_date: Date;
}

export interface PartyVote {
	party: string;
	infavor: boolean;
}

export interface VoteResultFilter {
	is_finished: boolean | null;
	is_named_vote: boolean | null;
	accepted: string | null;
	simple_majority: boolean | null;
	legis_period: string | null;
	is_law: boolean | null;
	vote_type: string | null;
	topics: string[] | null;
	party_votes: PartyVote[] | null;
	is_urgent: boolean | null;
}

export interface DelegateFilter {
	search_value: string | null;
	legis_period: string | null;
	day_offset: number | null;
}

export interface StanceTopicScore {
	topic: string;
	score: number;
}

export interface GovPropFilter {
	legis_period: string | null;
	has_vote_result: boolean | null;
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
