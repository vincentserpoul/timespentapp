<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  import type { Action } from "../../timespent/bindings/Action";
  export let labels: string[];
  export let action_percents: [string, number[]];
  export let project_percents: [string, number[]];

  let lines;

  const data = {
    labels: labels,
    datasets: action_percents.concat(project_percents).map(([name, vals]) => {
      return {
        label: name,
        data: vals,
        borderWidth: 3,
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
          labels: {
            usePointStyle: true,
            padding: 20,
            font: {
              size: 14,
            },
          },
        },
        title: {
          display: true,
          text: "My Personal Portfolio",
        },
      },
    },
  };

  onMount(async () => {
    const ctx = lines.getContext("2d");
    // Initialize chart using default config set
    var myChart = new Chart(ctx, config);
  });
</script>

<div class="card bg-gradient-info">
  <canvas id="myChart" height="100" bind:this={lines} />
</div>
