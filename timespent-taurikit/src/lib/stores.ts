import { writable } from 'svelte/store';

// Scale selection
import type { Scale } from '../../timespent/bindings/Scale';
import type { Filter } from '../../timespent/bindings/Filter';

export const selected_scale = writable<Scale>('Day');

export const filter = writable<Filter>({} as Filter);
