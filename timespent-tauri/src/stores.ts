import { writable } from "svelte/store";

// Scale selection
import type { Scale } from "../../timespent/bindings/Scale";

export const selected_scale = writable<Scale>("Day");
