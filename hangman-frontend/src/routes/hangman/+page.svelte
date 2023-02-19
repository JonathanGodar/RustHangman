<script lang="ts">
	import { Clues, type Clue } from './clue';
	import ClueInterface from './ClueInterface.svelte';
	import WordLengthInterface from './WordLengthInterface.svelte';

	/* function handle_key(evt: KeyboardEvent) { */
	/* 	if (evt.code === 'Enter' && evt.shiftKey) { */
	/* 		clues = new Clues(); */
	/* 	} else if (evt.key[0].toUpperCase() == evt.key[0]) { */
	/* 		clues.add_clue({ */
	/* 			letter: evt.key[0].toLowerCase(), */
	/* 			at_positions: [] */
	/* 		}); */
	/* 	} else { */
	/* 		clues.add_clue(); */
	/* 	} */

	/* 	console.log(evt); */
	/* 	clues = clues; */
	/* 	evt.preventDefault(); */
	/* } */

	/* let not_in_word: string[] = []; */
	/* let word: string[] = []; */
	/* let word_length: undefined | number = []; */

	/* let clues = new Clues(); */
	/* <div> */
	/* 	<input on:keypress={handle_key} /> */
	/* </div> */

	let wordLength: number | undefined = undefined;
	function onLength(evt: CustomEvent<number>) {
		wordLength = evt.detail;
	}
</script>

{#if wordLength == undefined}
	<WordLengthInterface on:lengthInput={onLength} />
{:else}
	<ClueInterface {wordLength} />
{/if}
