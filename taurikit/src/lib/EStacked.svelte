<script lang="ts">
	import ECharts from './ECharts.svelte';

	export let labels: string[] = [];
	export let activity_percents: [string, number[]][] = [];
	export let idchart: string;

	import stringToColor from '../lib/stringToColor';

	let option: echarts.EChartsOption;

	$: {
		option = {
			// title: {
			// 	text: 'Stacked Area Chart'
			// },
			tooltip: {
				trigger: 'axis',
				axisPointer: {
					type: 'cross'
				}
			},
			// color: activity_percents.map((ap) => stringToColor(ap[0])),
			legend: {
				data: activity_percents.map((ap) => ap[0]),
				textStyle: {
					color: '#fff'
				}
			},
			xAxis: [
				{
					type: 'category',
					boundaryGap: false,
					data: labels
				}
			],
			yAxis: [
				{
					type: 'value'
				}
			],
			series: activity_percents.map((ap) => {
				return {
					name: ap[0],
					type: 'line',
					stack: 'Total',
					areaStyle: {},
					smooth: true,
					emphasis: {
						focus: 'series'
					},
					data: ap[1]
				};
			})
		};
	}
</script>

{#if activity_percents.length > 0}
	<div id={idchart} class="chart">
		<ECharts {idchart} {option} />
	</div>
{/if}

<style>
	.chart {
		height: 30vh;
		width: 60vw;
	}
</style>
