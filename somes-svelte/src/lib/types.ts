export interface Delegate {
	id: number;
	name: string;
	party: string;
	image_url: string;
	constituency: string;
	council: string;
	seat_row: number | null;
	seat_col: number | null;
	gender: string | null;
	is_active: boolean | null;
	birthdate: string;
}

export interface LegislativeInitiative {
	id: string;
	ityp: string;
	gp: string;
	title: string;
	description: string;
	accepted: boolean;
	created_at: Date;
}

export interface Vote {
	party: string;
	fraction: number;
	infavor: boolean;
	legislative_initiatives_id: string;
}

export interface Speech {
	delegate_id: number;
	infavor: boolean;
}

export interface VoteResult {
	legislative_initiative: LegislativeInitiative;
	votes: Vote[];
	speeches: Speech[];
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

export interface Party {
	name: string;
	color: string;
	fraction: number;
}

export interface HasError {
	error: string;
}

export interface JWTInfo {
	access_token: string;
}

export interface UserInfo {
	username: string;
	email: string;
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
