<script context="module">
	export async function load({ params, session, fetch }) {
		console.log(params);
		const sounds_url = `${session.api_base_url}/sounds/${params.name}`;
		const response = await fetch(sounds_url);
		const sound = await response.json();
		return {
			props: {
				sound
			}
		};
	}
</script>

<script>
	export let sound;
</script>

<h1>{sound.name}</h1>
