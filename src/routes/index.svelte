<script>
	const sounds = [
		{ name: 'Arthur', file: '2. Arthur.mp3', volume: 1 },
		{ name: 'Emotional damage', file: '3. Emotional Damage.mp3' },
		{ name: 'Kais', file: '4. Kais.mp3', volume: 0.9 },
		{ name: 'Je prout', file: '5. Je Prout.mp3' },
		{ name: 'Leviosa', file: '6. Leviosa.mp3' },
		{ name: 'Sound', file: '7. Sound.mp3', volume: 0.8 },
		{ name: 'Ça a commencé ?', file: '8. Tout de suite la ca a commence.mp3', volume: 1 },
		{ name: 'Toute de suite ?', file: '9. Tout de suite la.mp3', volume: 1 },
		{ name: 'Whooa', file: '10. Whooa.mp3', volume: 1 }
	];
	let player;

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
			background-color: #26547c;
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
		max-width: 1280px;
		margin: auto;

		/*background-color: black;*/
	}
	.board {
		display: flex;
		flex-wrap: wrap;
	}
	.sound-item {
		border-radius: 1rem;
		margin: 1rem;
		padding: 2rem;
		height: 10rem;
		width: 10rem;
		background-color: #55d6be;
		text-align: center;
	}
	.sound-item:hover {
		background-color: #7cea9c;
	}
</style>
