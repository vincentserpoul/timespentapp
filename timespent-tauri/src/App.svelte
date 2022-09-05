<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";

  import ScalesComponent from "./Scales.svelte";
  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";

  import { selected_scale } from "./stores";

  // When using the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  async function getGraph(): Promise<[Number, ScaleXSegments, YActivities]> {
    let [scale_x_segments, y_activities]: [ScaleXSegments, YActivities] =
      await invoke("get_graph", {});
    let total_minutes_all = Number(y_activities.scale_total_minutes["All"][0]);
    return [total_minutes_all, scale_x_segments, y_activities];
  }

  async function getFilter(): Promise<[ActivitiesAggregate, Filter]> {
    let res = await invoke("get_filter", {});

    return [res[0] as ActivitiesAggregate, res[1] as Filter];
  }
</script>

<main>
  <div id="scales">
    <ScalesComponent />
  </div>
  {#await getGraph() then [total_minutes_all, scale_x_segments, y_activities]}
    <div id="charts">
      <GraphComponent
        selected_scale={$selected_scale}
        {total_minutes_all}
        {scale_x_segments}
        {y_activities}
      />
    </div>
  {/await}
  {#await getFilter() then [activitiesAggregate, filter]}
    <div id="filter">
      <FilterComponent {activitiesAggregate} {filter} />
    </div>
  {/await}
</main>

<style>
  main {
    padding: 2em;
  }

  #filter {
    margin-top: 2em;
  }
</style>
