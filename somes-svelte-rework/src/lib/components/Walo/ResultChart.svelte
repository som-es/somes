<script lang="ts">
	import { onMount } from 'svelte';
	import ApexCharts, { type ApexOptions } from 'apexcharts';
	import { partyToColor } from '$lib/partyColor';

	export let dataNoParty: number[] = [];

	const [spoe, gruene, neos, fpoe, oevp] = dataNoParty;

	const dataParty: [number, string][] = [
		[spoe, 'SPÖ'],
		[gruene, 'GRÜNE'],
		[neos, 'NEOS'],
		[fpoe, 'FPÖ'],
		[oevp, 'ÖVP']
	];

	dataParty.sort((a, b) => b[0] - a[0]);

	const scores = dataParty.map((a) => a[0]);
	const labels = dataParty.map((a) => a[1]);

	let chart: ApexCharts | null = null;

	let chartOptions: ApexOptions = {
		series: [
			{
				data: scores
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
		colors: labels.map(partyToColor),
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
                return ''
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
</script>

<div class=" text-black" id="chart"></div>

<style>
	#chart {
		max-width: 650px;
		margin: auto;
	}
</style>
