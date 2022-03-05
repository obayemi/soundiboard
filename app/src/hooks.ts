/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(event) {
	const api_base_url = process.env.API_URL;
	return {
		api_base_url
	};
}
