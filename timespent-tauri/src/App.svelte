<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";

  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";

  let total_minutes_all: Number;
  let scale_x_segments: ScaleXSegments;
  let labels: string[];
  let y_activities: YActivities;
  let activitiesAggregate: ActivitiesAggregate;

  import { getGraph as tauriGetGraph } from "./commands";
  async function syncGraph() {
    [total_minutes_all, scale_x_segments, y_activities] = await tauriGetGraph();
  }

  import { getFilter as tauriGetFilter } from "./commands";
  async function syncFilter() {
    let retrieved_filter: Filter;
    [activitiesAggregate, retrieved_filter] = await tauriGetFilter();

    filter.set(retrieved_filter);
  }

  import { filter } from "./stores";
  import { applyFilter as tauriApplyFilter } from "./commands";
  async function applyFilter() {
    let retrieved_filter: Filter;
    [activitiesAggregate, retrieved_filter] = await tauriApplyFilter($filter);

    filter.set(retrieved_filter);
  }

  import { onMount } from "svelte";
  onMount(async () => {
    await syncGraph();
    await syncFilter();
  });

  import { selected_scale } from "./stores";

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
      <FilterComponent {labels} {activitiesAggregate} {applyFilter} />
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
