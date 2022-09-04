<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";
  import stringToColor from "./stringToColor";

  export let activity_percents: { labels: string[]; values: number[] };

  let doughnut;

  const data = {
    labels: activity_percents.labels,
    datasets: [
      {
        data: activity_percents.values,
        backgroundColor: activity_percents.labels.map(stringToColor),
      },
    ],
  };
  const config = {
    type: "doughnut",
    data: data,
    options: {
      borderRadius: "10",
      responsive: true,
      cutout: "20%",
      spacing: 1,
      plugins: {
        legend: {
          position: "bottom",
          display: false,
        },
      },
    },
  };

  onMount(async () => {
    const ctx = doughnut.getContext("2d");
    // Initialize chart using default config set
    var myChart = new Chart(ctx, config);
  });
</script>

<div class="card bg-gradient-info">
  <canvas id="myChart" height="100" bind:this={doughnut} />
</div>
