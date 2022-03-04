<script>
	import { browser } from '$app/env';

	const sounds = [
		{ name: 'Arthur', file: '2. Arthur.mp3', volume: 1 },
		{ name: 'Emotional damage', file: '3. Emotional Damage.mp3' },
		{ name: 'Kais', file: '4. Kais.mp3', volume: 0.9 },
		{ name: 'Je prout', file: '5. Je Prout.mp3' },
		{ name: 'Leviosa', file: '6. Leviosa.mp3' },
		{ name: 'Sound', file: '7. Sound.mp3', volume: 0.8 },
		{ name: 'Ça a commencé ?', file: '8. Tout de suite la ca a commence.mp3', volume: 1 },
		{ name: 'Toute de suite ?', file: '9. Tout de suite la.mp3', volume: 1 },
		{ name: 'Whooa', file: '10. Whooa.mp3', volume: 1 },
		{ name: 'Oula', file: 'oula.mp3', volume: 1 },
		{ name: 'Yo glue', file: 'yo glue.mp3', volume: 1 },
	];

	let player;
    let cheatCode = [];

	if (browser) {
		document.onkeypress = function (event) {
			const char = typeof event !== 'undefined' ? event.keyCode : event.which;

                if (!isNaN(char)) {
                        console.log(char, +char + 11);
                        play(sounds[(+char + 11) % 10]);
                    }

                // queue structure: keep latest 5 key presses
                if (cheatCode.length >= 4) {
                        [, ...cheatCode] = cheatCode
                    }

                cheatCode = [...cheatCode, String.fromCharCode(char)]

                        console.log(cheatCode)
                // check cheat
                if (cheatCode.toString() === 'p,p,s,p') {
                        play({file:"ppsp.mp3"})
                    }
            };
	}

	function play(sound) {
		console.log(sound);
		if (player) {
			player.pause();
		}
		player = new Audio(sound.file);
		player.volume = sound.volume || 0.75;
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
		margin: 1rem;
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
