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
      borderRadius: "2",
      responsive: true,
      cutout: "50%",
      spacing: 0,
      borderWidth: 0,
      plugins: {
        legend: {
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
