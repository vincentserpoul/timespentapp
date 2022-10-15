<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";

  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";

  let total_minutes_all: Number;
  let scale_x_segments: ScaleXSegments;
  let y_activities: YActivities;

  import { getGraph as tauriGetGraph } from "./commands";
  async function syncGraph() {
    [total_minutes_all, scale_x_segments, y_activities] = await tauriGetGraph();
  }

  let all_x_labels: ScaleXSegments;
  let all_filter: Filter;

  import { getFilter as tauriGetFilter } from "./commands";
  async function syncFilter() {
    let applied_filter: Filter;
    [all_x_labels, all_filter, applied_filter] = await tauriGetFilter();

    filter.set(applied_filter);
  }

  import { filter } from "./stores";
  import { applyFilter as tauriApplyFilter } from "./commands";
  async function applyFilter() {
    await tauriApplyFilter($filter);

    await syncGraph();
  }

  import { onMount } from "svelte";
  onMount(async () => {
    await syncGraph();
    await syncFilter();
  });

  import { selected_scale } from "./stores";
  import { displayedXLabels } from "./display";
</script>

<main>
  {#if total_minutes_all > 0}
    <div id="charts">
      <GraphComponent
        selected_scale={$selected_scale}
        {total_minutes_all}
        labels={displayedXLabels(scale_x_segments[$selected_scale])}
        {y_activities}
      />
    </div>
    <div id="filter">
      <FilterComponent {all_x_labels} {all_filter} {applyFilter} />
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
