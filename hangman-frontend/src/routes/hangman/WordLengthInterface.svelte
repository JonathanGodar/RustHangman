<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	let len: number = 0;
	const dispatch = createEventDispatcher();
</script>

<form
	on:submit={(evt) => {
		dispatch('lengthInput', len);
		evt.preventDefault();
	}}
>
	<label for="length">Hur långt är ditt ord?</label>
	<input type="number" min="1" bind:value={len} />
	<input type="submit" />
</form>
