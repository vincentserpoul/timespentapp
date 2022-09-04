<script lang="ts">
  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";
  import type { Scale } from "../../timespent/bindings/Scale";

  let selected_scale: Scale = "Day";
</script>

<main>
  <div id="filter">
    <FilterComponent />
  </div>
  <div id="charts">
    <GraphComponent {selected_scale} />
  </div>
</main>

<style>
</style>
