<script lang="ts">
	import { onMount } from 'svelte';
	import * as echarts from 'echarts';

	export let labels: string[] = [];
	export let activity_percents: [string, number[]][] = [];
	export let idchart: string;

	import stringToColor from '../lib/stringToColor';
	type EChartsOption = echarts.EChartsOption;

	// Render line chart and components.
	onMount(async () => {
		const options: EChartsOption = {
			// title: {
			// 	text: 'Stacked Area Chart'
			// },
			// tooltip: {
			// 	trigger: 'axis',
			// 	axisPointer: {
			// 		type: 'cross',
			// 		label: {
			// 			backgroundColor: '#6a7985'
			// 		}
			// 	}
			// },
			color: activity_percents.map((ap) => stringToColor(ap[0])),
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

		const chart = echarts.init(document.getElementById(idchart));

		chart.setOption(options);
	});
</script>

<div id={idchart} class="chart" />

<style>
	.chart {
		height: 30vh;
		width: 60vw;
	}
</style>
