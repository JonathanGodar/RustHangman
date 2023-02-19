export type Clue = {
	at_positions: number[];
	letter: string;
}


export class Clues {
	clues: Clue[] = [];
	constructor() { }

	add_clue(clue: Clue): Clue | undefined {
		let removed = this.remove_clue(clue.letter);

		this.clues.push(clue);
		return removed;
	}

	remove_clue(letter: string): Clue | undefined {
		let idx = this.find_clue_idx(letter);
		if (idx === -1) return undefined;

		let deleted = this.clues.splice(idx, 1);

		return deleted.length === 1 ? deleted[0] : undefined;
	}


	find_clue(letter: string): Clue | undefined {
		let idx = this.find_clue_idx(letter);
		if (idx === -1) return undefined;
		return this.clues[idx];
	}


	find_clue_idx(letter: string): number {
		return this.clues.findIndex((c) => c.letter === letter);
	}
}
