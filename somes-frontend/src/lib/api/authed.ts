import type {
	DelegateFavo,
	ExtendedUserInfo,
	HasError,
	JWTInfo,
	LegisInitFavo,
	LoginResponseError,
	MailSendInfo,
	Quiz,
	UniqueTopic
} from '$lib/types';
import { address, fetchSavely, justPost, url } from './api';
import { getParliament, type Parliament } from './parliament';
import { jwtStore } from '$lib/caching/stores/stores.svelte';

export async function getWithAuth<T>(
	route: string,
	parliament: Parliament = getParliament()
): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${parliament}/${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			}
		})
	);
}

export async function putWithAuth<T>(
	route: string,
	body: any,
	parliament: Parliament = getParliament()
): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${parliament}/${route}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			},
			body: JSON.stringify(body)
		})
	);
}

export async function postWithAuth<T>(
	route: string,
	body: any,
	parliament: Parliament = getParliament()
): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${parliament}/${route}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			},
			body: JSON.stringify(body)
		})
	);
}

export async function deleteWithAuth<T>(
	route: string,
	body: any | undefined,
	parliament: Parliament = getParliament()
): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	let newBody: string | undefined;
	if (body) {
		newBody = JSON.stringify(body);
	} else {
		newBody = undefined;
	}
	return fetchSavely(() =>
		fetch(`${url}${parliament}/${route}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			},
			body: newBody
		})
	);
}

export async function addUserTopic(uniqueTopic: UniqueTopic): Promise<null | HasError> {
	return postWithAuth('v1/user/topic_selection', uniqueTopic);
}

export async function removeUserTopic(uniqueTopic: UniqueTopic): Promise<null | HasError> {
	return deleteWithAuth('v1/user/topic_selection', uniqueTopic);
}

export async function getUserTopics(
    parliament: Parliament = getParliament(),
): Promise<UniqueTopic[] | HasError> {
	return getWithAuth('v1/user/topic_selection', parliament);
}

export async function updateDelegateFavo(delegateFavo: DelegateFavo): Promise<null | HasError> {
	return putWithAuth('v1/user/bookmark/delegate', delegateFavo);
}

export async function addDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return postWithAuth('v1/user/bookmark/delegate', uniqueTopic);
}

export async function removeDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return deleteWithAuth('v1/user/bookmark/delegate', uniqueTopic);
}

export async function getFavoDelegates(
    parliament: Parliament = getParliament(),
): Promise<DelegateFavo[] | HasError> {
	return getWithAuth('v1/user/bookmark/delegate', parliament);
}

export async function addLegisInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return postWithAuth('v1/user/bookmark/vote_result', uniqueTopic);
}

export async function removeLegisInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return deleteWithAuth('v1/user/bookmark/vote_result', uniqueTopic);
}

export async function getFavoLegisInits(
    parliament: Parliament = getParliament(),
): Promise<LegisInitFavo[] | HasError> {
	return getWithAuth('v1/user/bookmark/vote_result', parliament);
}

export async function delete_account(): Promise<null | HasError> {
	return deleteWithAuth('v1/user/delete', undefined);
}

export async function getMailSendInfo(
    parliament: Parliament = getParliament(),
): Promise<MailSendInfo | HasError> {
	return getWithAuth('v1/user/send_mail_info', parliament);
}

export async function getUser(): Promise<ExtendedUserInfo | HasError> {
	return getWithAuth('v1/user');
}

export async function userInit(parliament: Parliament = getParliament()): Promise<null | HasError> {
	return getWithAuth('v1/user/init', parliament);
}

export async function getQuizzes(): Promise<Quiz[] | HasError> {
	return getWithAuth('quizzes');
}

export async function updateMailSendInfo(mailSendInfo: MailSendInfo, parliament: Parliament = getParliament()): Promise<null | HasError> {
	return putWithAuth('v1/user/send_mail_info', mailSendInfo, parliament);
}

export async function renew_token(): Promise<JWTInfo | HasError> {
	return postWithAuth<JWTInfo>('v1/user/renew_token', {});
}

export async function login(
	email: string,
	password: string | null,
	hash_email: boolean | null
): Promise<JWTInfo | HasError | LoginResponseError> {
	return justPost('v1/user/login', { email, password, hash_email });
}

export async function change_email(new_email: string): Promise<any | HasError> {
	return postWithAuth('v1/user/change_email', { new_email });
}

export async function verify_email_change(new_email: string, otp: string): Promise<any | HasError> {
	return postWithAuth('v1/user/verify_email_change', { new_email, otp });
}

export async function anonymize_email(): Promise<any | HasError> {
	return postWithAuth('v1/user/anonymize_email', {});
}
