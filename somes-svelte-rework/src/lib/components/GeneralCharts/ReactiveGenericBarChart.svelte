<script lang="ts">
	import { onMount } from 'svelte';
	import ApexCharts, { type ApexOptions } from 'apexcharts';
	import type { ChartData } from '$lib/types';
	export let chartData: ChartData[];
	export let title: string;
	export let legend: boolean = false;

	export let height: number = 490;
	export let horizontalBars: boolean = false;

	let chart: ApexCharts | null = null;
	let chartOptions: ApexOptions;

	$: {
		const labels = chartData.map((del) => `${del.label}`);
		const isBlack = $modeCurrent;

		chartOptions = {
			series: [
				{
					data: chartData.map((del) => del.data)
				}
			],

			title: {
				text: title,
				align: 'center',
				floating: true
			},
			chart: {
				animations: {
					enabled: false
				},
				type: 'bar',
				height
			},
			plotOptions: {
				bar: {
					borderRadius: 4,
					borderRadiusApplication: 'end',
					horizontal: horizontalBars,
					distributed: true,
					dataLabels: {
						position: 'bottom'
					}
				}
			},
			colors: chartData.map((del) => {
				return del.color;
			}),
			dataLabels: {
				textAnchor: 'middle',
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
				categories: labels,
				labels: {
					style: {
						colors: labels.map(() => {
							if (isBlack) {
								return 'black';
							} else {
								return 'white';
							}
						}),
						fontSize: '12px'
					}
				}
			},

			yaxis: {
				labels: {
					style: {
						colors: labels.map(() => {
							if (isBlack) {
								return 'black';
							} else {
								return 'white';
							}
						}),
						fontSize: '12px'
					}
				}
			},

			legend: {
				show: legend
			}
		};

		if (chart) {
			chart.updateOptions(chartOptions);
		}
	}

	let chartElement: Element;
	onMount(() => {
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

<div bind:this={chartElement} class="text-black"></div>

<style>
	#chart {
		min-width: 100%;
	}
</style>
