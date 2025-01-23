<script lang="ts">
	import { onMount } from 'svelte';
	import ApexCharts, { type ApexOptions } from 'apexcharts';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateData  } from '$lib/types';



	export let delegateData: DelegateData[];

	let chart: ApexCharts | null = null;
	let chartOptions: ApexOptions;

	$: {
		const labels = delegateData.map(del => del.name);

		chartOptions = {
			series: [
				{
					data: delegateData.map(del => del.data)
				}
			],
			chart: {
				type: 'bar',
				height: 350
			},
			plotOptions: {
				bar: {
					borderRadius: 4,
					borderRadiusApplication: 'end',
					horizontal: true,
					distributed: true,
					dataLabels: {
						position: 'bottom'
					}
				}
			},
			colors: delegateData.map(del => {
				return partyToColor(del.party)
			}),
			dataLabels: {
				offsetX: 14,
				textAnchor: 'start',
				enabled: true
			},
			tooltip: {
				x: {
					show: true
				},
				y: {
					title: {
						formatter: function () {
							return '';
						}
					}
				}
			},
			grid: {
				borderColor: '#e0e0e0',
				strokeDashArray: 1,
				row: {
					colors: ['#f3f3f3', '#ffffff'],
					opacity: 0.5
				},
				column: {
					colors: ['#f3f3f3', '#ffffff'],
					opacity: 0.5
				}
			},
			xaxis: {
				categories: labels
			}
		};

		if (chart) {
			chart.updateOptions(chartOptions);
		}
	}

	onMount(() => {
		const chartElement = document.querySelector('#chart');

		if (chartElement) {
			chart = new ApexCharts(chartElement, chartOptions);
			chart.render();
		}

		return () => {
			chart?.destroy();
		};
	});

	$: if (chart && chartOptions) {
		chart.updateOptions(chartOptions);
	}
</script>

<div class="text-black" id="chart"></div>

<style>
	#chart {
		max-width: 650px;
		margin: auto;
	}
</style>
