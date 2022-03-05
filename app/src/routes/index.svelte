<script>
	import { browser } from '$app/env';

	let player;
	let cheatCode = [];
	export let sounds = [];

	if (browser) {
		document.onkeypress = function (event) {
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
		};
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

<svelte:head>
	<title>Sensiboard</title>
	<style>
		html,
		body {
			height: 100%;
		}
		body {
			background-color: #191919;
		}
	</style>
</svelte:head>

<div class="page">
	<h1>SensiBoard</h1>
	<div class="board">
		{#each sounds as sound}
			<div class="sound-item" on:click={() => play(sound)}>{sound.name}</div>
		{/each}
	</div>
</div>

<style>
	.page {
		color: #ecdbba;
		max-width: 1280px;
		margin: auto;

		/*background-color: black;*/
	}
	.board {
		display: flex;
		flex-wrap: wrap;
	}
	.sound-item {
		box-sizing: border-box;
		font-weight: bold;
		font-size: 1.5rem;
		border-radius: 5px;
		margin: 0.3rem;
		padding: 2rem;
		height: 10rem;
		width: 10rem;
		background-color: #2d4263;
		text-align: center;
	}
	.sound-item:hover {
		background-color: #c84b31;
	}
</style>
