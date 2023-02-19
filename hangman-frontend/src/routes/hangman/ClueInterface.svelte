<script lang="ts">
	import { onMount } from 'svelte';

	export let wordLength: number;
	let cursor = 0;
	let word: string[] = Array(wordLength).fill('');
	let not_in_word: string[] = [];

	function onKeyPress(evt: KeyboardEvent) {
		if (evt.key == 'Enter' && evt.shiftKey) {
			word = Array(wordLength).fill('');
			not_in_word = [];
		} else if (evt.shiftKey) {
			not_in_word = [...not_in_word, evt.key.toLowerCase()];
		} else {
			word[cursor] = evt.key;

			incrementCursor();
		}

		console.log(evt);
		evt.preventDefault();
	}

	function onKeyDown(evt: KeyboardEvent) {
		if (evt.key == 'Tab') {
			incrementCursor();
			evt.preventDefault();
		} else if (evt.key == 'Backspace') {
			word[cursor] = '';
			cursor = Math.max(cursor - 1, 0);
		}
	}

	function incrementCursor() {
		cursor += 1;
		if (cursor >= wordLength) cursor = 0;
	}

	onMount(async () => {
		let a = await fetch(`http://localhost:8000/hangman`)
			.then((r) => r.json())
			.then((d) => console.log(d));

		console.log(a);
	});
</script>

<input on:keypress={onKeyPress} on:keydown={onKeyDown} />

<h1>{word}</h1>
<h1>{not_in_word}</h1>
<h1>{cursor}</h1>
