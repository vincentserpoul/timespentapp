<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";
  import stringToColor from "./stringToColor";

  export let labels: string[];
  export let activity_percents: [string, number[]];

  let lines;

  const data = {
    labels: labels,
    datasets: activity_percents.map(([name, vals]) => {
      return {
        label: name,
        data: vals,
        borderWidth: 4,
        borderColor: stringToColor(String(name)),
      };
    }),
  };
  const config = {
    type: "line",
    data: data,
    options: {
      borderRadius: "10",
      responsive: true,
      cutout: "95%",
      spacing: 2,
      plugins: {
        legend: {
          position: "bottom",
          display: true,
          align: "center",
          padding: 10,
          labels: {
            usePointStyle: true,
            pointStyle: "star",
            padding: 20,
            color: "white",
            font: {
              size: 20,
            },
          },
        },
      },
    },
  };

  let myChart;

  onMount(async () => {
    const ctx = lines.getContext("2d");
    // Initialize chart using default config set
    myChart = await new Chart(ctx, config);
  });

  $: {
    if (myChart !== undefined && myChart.data !== undefined) {
      myChart.data.labels = labels;

      myChart.data.datasets = activity_percents.map(([name, vals]) => {
        return {
          label: name,
          data: vals,
          borderWidth: 4,
          borderColor: stringToColor(String(name)),
        };
      });

      myChart.update();
    }
  }
</script>

<canvas id="myChart" height="100" bind:this={lines} />
