import * as cookie from 'cookie';
import { userFromJwt } from '$lib/auth';

function parseCookie(str) {
	return str
		.split(';')
		.map((v) => v.split('='))
		.reduce((acc, v) => {
			acc[decodeURIComponent(v[0].trim())] = decodeURIComponent(v[1].trim());
			return acc;
		}, {});
}

/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(event) {
	const api_base_url = process.env.API_URL;
	return {
		api_token: event.locals.api_token,
		user: event.locals.user,
		api_base_url
	};
}

/** @type {import('@sveltejs/kit').handle} */
export async function handle({ event, resolve }) {
	const cookie = event.request.headers.get('cookie');

	event.locals.api_token = null;
	event.locals.user = null;

	if (cookie) {
		const { API_JWT } = parseCookie(cookie);
		if (API_JWT) {
			event.locals.api_token = API_JWT;
			event.locals.user = userFromJwt(API_JWT);
		}
	}
	//console.log(event.locals.jwt);
	//console.log('aaa', event);
	//console.log('bbb', event.request);
	//console.log(event, event.request.headers.get('cookie'));

	const response = await resolve(event);
	return response;
}
