<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import ApexCharts, { type ApexOptions } from 'apexcharts';
	import { partyToColor } from '$lib/partyColor';
	import type { DelegateData  } from '$lib/types';

	export let delegateData: DelegateData[];

	export let title: string;
	export let height: number;

	let chartElement: Element;

	let chart: ApexCharts | null = null;
	let chartOptions: ApexOptions;

	$: {
		delegateData = delegateData.slice(0, 100)
		const labels = delegateData.map(del => `${del.name} (${del.party})`);

		chartOptions = {
			series: [
				{
					data: delegateData.map(del => del.data)
				}
			],

			title: {
				text: title,
				align: 'center',
				floating: true
        	},
			chart: {
				type: 'bar',
				height: height,
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

			},

			legend: {
				show: false
			}
		};

		if (chart) {
			chart.updateOptions(chartOptions);
		}
	}

	onMount(() => {
		initChart(chartElement)
	});

	const initChart = (chartElement: Element) => {
		//const chartElement = document.querySelector(chartId);
		console.log(chartElement);
		if (chartElement && !chart) {
			chart = new ApexCharts(chartElement, chartOptions);
			chart.render();
		}
	};

	onDestroy(() => {
		chart?.destroy();
	})

	$: initChart(chartElement)

	$: if (chart && chartOptions) {
		chart.updateOptions(chartOptions);
	}
</script>

<div bind:this={chartElement} class="text-black chart"></div>

<style>
	.chart {
		max-width: 650px;
	}
</style>
