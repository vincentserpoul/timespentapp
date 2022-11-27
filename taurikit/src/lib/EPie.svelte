<script lang="ts">
	import { onMount } from 'svelte';
	import * as echarts from 'echarts';

	export let activity_percents: { name: string; value: number }[];
	export let idpie: string;

	import stringToColor from '../lib/stringToColor';
	type EChartsOption = echarts.EChartsOption;

	// Render line chart and components.
	onMount(async () => {
		const options: EChartsOption = {
			legend: {
				orient: 'vertical',
				left: 'left',
				textStyle: {
					color: '#fff'
				}
			},

			color: activity_percents.map((v) => stringToColor(v.name)),
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

		const chart = echarts.init(document.getElementById(idpie));

		chart.setOption(options);
	});
</script>

<div id={idpie} class="pie" />

<style>
	.pie {
		height: 30vh;
		width: 30vw;
	}
</style>
