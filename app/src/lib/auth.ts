import { writable, get } from 'svelte/store';
import { session } from '$app/stores';
import Cookies from 'js-cookie';

export function userFromJwt(token) {
	console.log(token);
	const decoded = JSON.parse(atob(token.split('.')[1]));
	return {
		id: decoded.id,
		username: decoded.username
	};
}

export function login(token) {
	Cookies.set('API_JWT', token, { sameSite: 'strict', expires: 7 });
	session.update((s) => ({ ...s, user: userFromJwt(token) }));
}

export function logout() {
	Cookies.remove('API_JWT');
	session.update((s) => ({ ...s, user: null }));
}
