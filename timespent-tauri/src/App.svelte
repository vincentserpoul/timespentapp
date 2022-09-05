<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";

  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";

  import { selected_scale } from "./stores";

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

  let total_minutes_all: Number;
  let scale_x_segments: ScaleXSegments;
  let labels: string[];
  let y_activities: YActivities;
  let activitiesAggregate: ActivitiesAggregate;
  let filter: Filter;

  import { onMount } from "svelte";
  onMount(async () => {
    [total_minutes_all, scale_x_segments, y_activities] = await getGraph();
    [activitiesAggregate, filter] = await getFilter();
  });

  $: {
    if (scale_x_segments !== undefined) {
      labels = scale_x_segments[$selected_scale].map(
        (x_segment) => x_segment.start_datetime
      );
    }
  }
</script>

<main>
  {#if total_minutes_all > 0}
    <div id="charts">
      <GraphComponent
        selected_scale={$selected_scale}
        {total_minutes_all}
        {labels}
        {y_activities}
      />
    </div>
    <div id="filter">
      <FilterComponent {labels} {activitiesAggregate} {filter} />
    </div>
  {/if}
</main>

<style>
  main {
    padding: 2em;
  }

  #filter {
    margin-top: 2em;
  }
</style>
