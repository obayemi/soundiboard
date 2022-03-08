<script context="module">
	export async function load({ session, fetch }) {
		const sounds_url = `${session.api_base_url}/sounds/`;
		const response = await fetch(sounds_url, {
			headers: {
				Authorization: `BEARER ${session.api_token}`
			}
		});
		const sounds = await response.json();
		return {
			props: {
				sounds
			}
		};
	}
</script>

<script>
	let player;
	let cheatCode = [];
	export let sounds = [];

	function handleKeydown(event) {
		const char = typeof event !== 'undefined' ? event.keyCode : event.which;

		if (!isNaN(char)) {
			play(sounds[(+char + 11) % 10]);
		}

		// queue structure: keep latest 5 key presses
		if (cheatCode.length >= 4) {
			[, ...cheatCode] = cheatCode;
		}

		cheatCode = [...cheatCode, String.fromCharCode(char)];

		// check cheat
		if (cheatCode.toString() === 'p,p,s,p') {
			play({ file_url: 'ppsp.mp3' });
		}
	}

	function play(sound) {
		if (player) {
			player.pause();
		}
		player = new Audio(sound.file_url);
		player.volume = sound.volume / 100;
		player.play();
	}
</script>

<svelte:window on:keydown={handleKeydown} />
<div class="flex flex-wrap gap-4 mt-8">
	{#each sounds as sound}
		<div
			class="rounded shadow-xl w-48 h-48 p-4 text-3xl font-bold text-center bg-purple-900 hover:bg-purple-700"
			on:click={() => play(sound)}
		>
			{sound.name}
		</div>
	{/each}
</div>
