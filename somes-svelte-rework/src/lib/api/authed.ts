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
import { address, fetchSavely, url } from './api';
import { jwtStore } from '$lib/caching/stores/stores';
import { get } from 'svelte/store';

export async function getWithAuth<T>(route: string): Promise<T | HasError> {
	const accessToken = get(jwtStore);
	if (accessToken == null) {
		return { error: 'No access token' };
	}
	return fetchSavely(() =>
		fetch(`${url}/${route}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			}
		})
	);
}

export async function putWithAuth<T>(route: string, body: any): Promise<T | HasError> {
	const accessToken = get(jwtStore);
	if (accessToken == null) {
		return { error: 'No access token' };
	}
	return fetchSavely(() =>
		fetch(`${url}${route}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			},
			body: JSON.stringify(body)
		})
	);
}

export async function postWithAuth<T>(route: string, body: any): Promise<T | HasError> {
	const accessToken = get(jwtStore);
	if (accessToken == null) {
		return { error: 'No access token' };
	}
	return fetchSavely(() =>
		fetch(`${url}${route}`, {
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
	body: any | undefined
): Promise<T | HasError> {
	const accessToken = get(jwtStore);
	if (accessToken == null) {
		return { error: 'No access token' };
	}
	let newBody: string | undefined;
	if (body) {
		newBody = JSON.stringify(body);
	} else {
		newBody = undefined;
	}
	return fetchSavely(() =>
		fetch(`${url}${route}`, {
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
	return postWithAuth('topic_selection', uniqueTopic);
}

export async function removeUserTopic(uniqueTopic: UniqueTopic): Promise<null | HasError> {
	return deleteWithAuth('topic_selection', uniqueTopic);
}

export async function getUserTopics(): Promise<UniqueTopic[] | HasError> {
	return getWithAuth('topic_selection');
}

export async function addDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return postWithAuth('favo_delegate', uniqueTopic);
}

export async function removeDelegateFavo(uniqueTopic: DelegateFavo): Promise<null | HasError> {
	return deleteWithAuth('favo_delegate', uniqueTopic);
}

export async function getFavoDelegates(): Promise<DelegateFavo[] | HasError> {
	return getWithAuth('favo_delegate');
}

export async function addLegisInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return postWithAuth('favo_legis_init', uniqueTopic);
}

export async function removeLegiInitFavo(uniqueTopic: LegisInitFavo): Promise<null | HasError> {
	return deleteWithAuth('favo_legis_init', uniqueTopic);
}

export async function getFavoLegisInits(): Promise<LegisInitFavo[] | HasError> {
	return getWithAuth('favo_legis_init');
}

export async function delete_account(): Promise<null | HasError> {
	return deleteWithAuth('delete_account', undefined);
}

export async function getMailSendInfo(): Promise<MailSendInfo | HasError> {
	return getWithAuth('send_mail_info');
}

export async function getUser(): Promise<ExtendedUserInfo | HasError> {
	return getWithAuth('user');
}

export async function getQuizzes(): Promise<Quiz[] | HasError> {
	return getWithAuth('quizzes');
}

export async function updateMailSendInfo(mailSendInfo: MailSendInfo): Promise<null | HasError> {
	return putWithAuth('send_mail_info', mailSendInfo);
}

export async function renew_token(): Promise<JWTInfo | HasError> {
	return postWithAuth<JWTInfo>('renew_token', {});
}

export async function login(
	email: string,
	password: string | null,
	hash_email: boolean | null
): Promise<JWTInfo | HasError | LoginResponseError> {
	return postWithAuth('login', { email, password, hash_email });
}
