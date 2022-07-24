<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import Project from "./Project.svelte";
  import Action from "./Action.svelte";

  export let filter: Filter;
</script>

<div>
  <h1>filter</h1>
  {filter.min_date}-{filter.max_date}:
  {#if filter}
    <ul>
      {#each filter.projects as project}
        <li>
          <Project {project} />
        </li>
      {/each}
    </ul>
  {/if}
  {#if filter}
    <ul>
      {#each filter.actions as action}
        <li>
          <Action {action} />
        </li>
      {/each}
    </ul>
  {/if}
</div>
