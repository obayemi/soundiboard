export async function get({ params }) {
	const api_base_url = process.env.API_URL;
	const sounds_url = `${api_base_url}/sounds/`;
	const response = await fetch(sounds_url);
	const sounds = await response.json();

	return {
		body: { api_base_url, sounds }
	};
}
