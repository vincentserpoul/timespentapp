<script lang="ts">
	import type { Scale } from '../../timespent/bindings/Scale';
	import type { ScaleXSegments } from '../../timespent/bindings/ScaleXSegments';

	const scales: Scale[] = ['Day', 'Week', 'Month', 'Year'];

	import { selected_scale } from './stores';
	function select_scale(s: Scale) {
		selected_scale.set(s);
	}
</script>

<ul class="scale-list">
	{#each scales as scale}
		<li
			class={scale === $selected_scale ? 'selected' : ''}
			href="#"
			on:click={() => {
				select_scale(scale);
			}}
		>
			{scale}
		</li>
	{/each}
</ul>

<style>
	ul.scale-list {
		list-style: none;
		list-style-type: none;
		text-align: center;
	}

	ul.scale-list li {
		display: inline-block;
		cursor: pointer;
		width: 7em;
		padding: 0.5em;
		text-transform: lowercase;
		margin: 0 0.5em 0 0.5em;
		border: 1px solid white;
		border-radius: 1em;
	}

	.selected {
		font-weight: bold;
		color: black;
		background-color: rgb(206, 148, 82);
	}
</style>
