<script lang="ts">
  import FilterComponent from "./Filter.svelte";
  import GraphComponent from "./Graph.svelte";
  import type { Scale } from "../../timespent/bindings/Scale";

  let selected_scale: Scale = "Day";
</script>

<main>
  <FilterComponent />
  <GraphComponent {selected_scale} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
