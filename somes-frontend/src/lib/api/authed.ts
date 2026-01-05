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
import { jwtStore } from '$lib/caching/stores/stores.svelte';

export async function getWithAuth<T>(route: string, country = 'at/'): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${country}${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			}
		})
	);
}

export async function putWithAuth<T>(route: string, body: any, country = 'at/'): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${country}${route}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			},
			body: JSON.stringify(body)
		})
	);
}

export async function postWithAuth<T>(route: string, body: any, country = 'at/'): Promise<T | HasError> {
	const accessToken = jwtStore.value;
	if (accessToken == null) {
		return { error: 'No access token', error_type: 'AuthError', field: 'MissingToken', meta: null };
	}
	return fetchSavely(() =>
		fetch(`${url}${country}${route}`, {
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
	country = 'at/'
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
		fetch(`${url}${country}${route}`, {
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

export async function getUserTopics(): Promise<UniqueTopic[] | HasError> {
	return getWithAuth('v1/user/topic_selection');
}

export async function addDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return postWithAuth('v1/user/bookmark/delegate', uniqueTopic);
}

export async function removeDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return deleteWithAuth('v1/user/bookmark/delegate', uniqueTopic);
}

export async function getFavoDelegates(): Promise<DelegateFavo[] | HasError> {
	return getWithAuth('v1/user/bookmark/delegate');
}

export async function addLegisInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return postWithAuth('v1/user/bookmark/vote_result', uniqueTopic);
}

export async function removeLegisInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return deleteWithAuth('v1/user/bookmark/vote_result', uniqueTopic);
}

export async function getFavoLegisInits(): Promise<LegisInitFavo[] | HasError> {
	return getWithAuth('v1/user/bookmark/vote_result');
}

export async function delete_account(): Promise<null | HasError> {
	return deleteWithAuth('v1/user/delete_account', undefined);
}

export async function getMailSendInfo(): Promise<MailSendInfo | HasError> {
	return getWithAuth('v1/user/send_mail_info');
}

export async function getUser(): Promise<ExtendedUserInfo | HasError> {
	return getWithAuth('v1/user');
}

export async function getQuizzes(): Promise<Quiz[] | HasError> {
	return getWithAuth('quizzes');
}

export async function updateMailSendInfo(mailSendInfo: MailSendInfo): Promise<null | HasError> {
	return putWithAuth('v1/user/send_mail_info', mailSendInfo);
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
