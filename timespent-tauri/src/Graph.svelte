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
  import All from "./All.svelte";

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

{#await getGraph() then [total_minutes, scaleXSegments, yActivities]}
  <div>
    <h1>all activities</h1>
    <All
      projects={Object.entries(
        yActivities.scale_projects_total_minutes["All"]
      ).map((project) => [
        project[0],
        Math.round((Number(project[1][0]) * 100) / Number(total_minutes)),
      ])}
      actions={Object.entries(
        yActivities.scale_actions_total_minutes["All"]
      ).map((action) => [
        action[0],
        Math.round((Number(action[1][0]) * 100) / Number(total_minutes)),
      ])}
    />
    <Lines
      labels={scaleXSegments[selected_scale].map(
        (x_segment) => x_segment.start_datetime
      )}
      project_percents={Object.entries(
        yActivities.scale_projects_total_minutes[selected_scale]
      ).map(([name, durations]) => [
        name,
        calcActivityPercents(
          yActivities.scale_total_minutes[selected_scale],
          durations
        ),
      ])}
      action_percents={Object.entries(
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
