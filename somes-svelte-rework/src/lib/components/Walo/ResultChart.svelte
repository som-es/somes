<script lang="ts">
    import { onMount } from 'svelte';
    import ApexCharts, { type ApexOptions } from 'apexcharts';  
  
    let chart: ApexCharts | null = null;
  
    let chartOptions: ApexOptions = {
      chart: {
        type: 'line',
        height: 350
      },
      series: [{
        name: 'Sales',
        data: [10, 41, 35, 51, 49, 62, 69, 91, 148]
      }],
      xaxis: {
        categories: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep']
      }
    };
  
    onMount(() => {
      const chartElement = document.querySelector("#chart");
  
      if (chartElement) {
        chart = new ApexCharts(chartElement, chartOptions);
        chart.render();
      }
  
      return () => {
        chart?.destroy();
      };
    });
  </script>
  
  <div id="chart"></div>
  
  <style>
    #chart {
      max-width: 650px;
      margin: auto;
    }
  </style>
  