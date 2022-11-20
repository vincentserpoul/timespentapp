<script lang="ts">
	import DatePicker from '@beyonk/svelte-datepicker/src/components/DatePicker.svelte';
	import { filter } from './stores';

	export let min_date;
	export let max_date;
	export let filtered_min_date;
	export let filtered_max_date;
	export let applyFilter: () => Promise<void>;

	let selected = [filtered_min_date, filtered_max_date];
</script>

<DatePicker
	start={min_date}
	end={max_date}
	range={true}
	{selected}
	on:range-selected={(e) => {
		filter.update((f) => {
			f.min_date = e.detail.from.toISOString().split('T')[0];
			f.max_date = e.detail.to.toISOString().split('T')[0];
			return f;
		});

		applyFilter();
	}}
/>
<br />
