<script lang="ts">
  import FilterComponent from "./Filter.svelte";
  import AggregatesComponent from "./Aggregates.svelte";

  import type { Filter } from "../../timespent/bindings/Filter";
  import type { Aggregates } from "../../timespent/bindings/Aggregates";

  export let filter: Filter;

  export let aggregates: Aggregates;

  // When using the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  invoke("get_curr_state", {})
    .then((res) => {
      filter = res[0] as Filter;
      aggregates = res[1] as Aggregates;
    })
    .catch((e) => console.error(e));
</script>

<main>
  {#if filter}
    <FilterComponent {filter} />
  {/if}
  {#if aggregates}
    <AggregatesComponent {aggregates} />
  {/if}
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
