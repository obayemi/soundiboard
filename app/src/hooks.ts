import * as cookie from 'cookie';

/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(event) {
	const api_base_url = process.env.API_URL;
	return {
		api_base_url
	};
}

/** @type {import('@sveltejs/kit').handle} */
export async function handle({ event, resolve }) {
	console.log('aaa', event);
	console.log('bbb', event.request);
	//console.log(event, event.request.headers.get('cookie'));

	const response = await resolve(event);
	return response;
}
