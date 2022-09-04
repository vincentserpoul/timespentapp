<script lang="ts">
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";

  // When using the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  async function getGraph(): Promise<[Number, ScaleXSegments, YActivities]> {
    let [scale_x_segments, y_activities]: [ScaleXSegments, YActivities] =
      await invoke("get_graph", {});
    let total_minutes_all = Number(y_activities.scale_total_minutes["All"][0]);
    return [total_minutes_all, scale_x_segments, y_activities];
  }

  export let selected_scale;

  import Lines from "./Lines.svelte";
  import Camembert from "./Camembert.svelte";

  function calcActivityPercents(
    total_minutes: number[],
    activity_minutes: number[]
  ): number[] {
    return activity_minutes.map((activity_minute, i) => {
      if (activity_minute == 0) {
        return 0;
      }
      return Math.round((activity_minute * 100) / total_minutes[i]);
    });
  }
</script>

<div class="grid-container">
  {#await getGraph() then [total_minutes, scaleXSegments, yActivities]}
    <div class="grid-item title"><h1>projects</h1></div>
    <div class="grid-item doughnut-projects">
      <Camembert
        activity_percents={Object.entries(
          yActivities.scale_projects_total_minutes["All"]
        )
          .map((activity) => [
            activity[0],
            Math.round((Number(activity[1][0]) * 100) / Number(total_minutes)),
          ])
          .reduce(
            (acc, activity) => {
              acc.labels.push(activity[0]);
              acc.values.push(activity[1]);
              return acc;
            },
            { labels: [], values: [] }
          )}
      />
    </div>
    <div class="grid-item graph-projects">
      <Lines
        labels={scaleXSegments[selected_scale].map(
          (x_segment) => x_segment.start_datetime
        )}
        activity_percents={Object.entries(
          yActivities.scale_projects_total_minutes[selected_scale]
        ).map(([name, durations]) => [
          name,
          calcActivityPercents(
            yActivities.scale_total_minutes[selected_scale],
            durations
          ),
        ])}
      />
    </div>
    <div class="grid-item title"><h1>actions</h1></div>
    <div class="grid-item doughnut-actions">
      <Camembert
        activity_percents={Object.entries(
          yActivities.scale_actions_total_minutes["All"]
        )
          .map((activity) => [
            activity[0],
            Math.round((Number(activity[1][0]) * 100) / Number(total_minutes)),
          ])
          .reduce(
            (acc, activity) => {
              acc.labels.push(activity[0]);
              acc.values.push(activity[1]);
              return acc;
            },
            { labels: [], values: [] }
          )}
      />
    </div>
    <div class="grid-item graph-actions">
      <Lines
        labels={scaleXSegments[selected_scale].map(
          (x_segment) => x_segment.start_datetime
        )}
        activity_percents={Object.entries(
          yActivities.scale_actions_total_minutes[selected_scale]
        ).map(([name, durations]) => [
          name,
          calcActivityPercents(
            yActivities.scale_total_minutes[selected_scale],
            durations
          ),
        ])}
      />
    </div>
  {/await}
</div>

<style>
  .grid-container {
    display: grid;
    gap: 10px;
    background-color: #2196f3;
    padding: 10px;
    border: 10px solid red;
  }

  .grid-item {
    background-color: rgba(255, 255, 255, 0.8);
    text-align: center;
    padding: 20px;
    font-size: 30px;
  }

  .title {
    grid-column: 1 / span 2;
    grid-row: 1;
  }

  .doughnut-projects {
    grid-column: 1;
    grid-row: 2;
  }

  .graph-projects {
    grid-column: 1;
    grid-row: 2;
  }

  .doughnut-actions {
    grid-column: 1;
    grid-row: 2;
  }

  .graph-actions {
    grid-column: 1;
    grid-row: 2;
  }
</style>
