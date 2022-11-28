<script lang="ts">
	import ECharts from './ECharts.svelte';

	export let activity_percents: { name: string; value: number }[];
	export let idpie: string;

	import stringToColor from '../lib/stringToColor';

	let option: echarts.EChartsOption;

	$: {
		option = {
			legend: {
				orient: 'vertical',
				left: 'left',
				textStyle: {
					color: '#fff'
				},
				type: 'scroll'
			},

			// color: activity_percents.map((v) => stringToColor(v.name)),
			series: [
				{
					name: 'Radius Mode',
					type: 'pie',
					radius: [20, 120],
					center: ['60%', '60%'],
					roseType: 'radius',
					itemStyle: {
						borderRadius: 10,
						borderColor: '#fff',
						borderWidth: 1
					},
					label: {
						show: false
					},
					emphasis: {
						label: {
							show: true,
							fontSize: 15,
							fontWeight: 'bold',
							color: '#fff'
						}
					},
					data: activity_percents
				}
			]
		};
	}
</script>

{#if activity_percents.length > 0}
	<div id={idpie} class="pie">
		<ECharts idchart={idpie} {option} />
	</div>
{/if}

<style>
	.pie {
		height: 30vh;
		width: 30vw;
	}
</style>
