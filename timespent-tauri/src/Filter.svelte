<script lang="ts">
  import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
  import type { Filter } from "../../timespent/bindings/Filter";

  export let activitiesAggregate: ActivitiesAggregate;
  export let filter: Filter;

  import ActivityButton from "./ActivityButton.svelte";
</script>

<div class="grid-container">
  <div class="grid-item title">
    <h1>filter</h1>
  </div>
  {#if filter}
    <div class="grid-item sub-title slider">
      <h2>time range ({activitiesAggregate[0]}-{activitiesAggregate[1]})</h2>
    </div>
    <div class="grid-item sub-values slider">d</div>
    <div class="grid-item sub-title projects"><h2>projects</h2></div>
    <div class="grid-item sub-values projects">
      <ul class="activity-filter">
        {#each activitiesAggregate[3] as project}
          <li>
            <ActivityButton
              activity={project}
              selected={filter.projects.includes(project)}
            />
          </li>
        {/each}
      </ul>
    </div>
    <div class="grid-item sub-title actions"><h2>actions</h2></div>
    <div class="grid-item sub-values actions">
      <ul class="activity-filter">
        {#each activitiesAggregate[2] as action}
          <li>
            <ActivityButton
              activity={action}
              selected={filter.actions.includes(action)}
            />
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  .grid-container {
    display: grid;
    padding: 1em;
    text-align: center;
    border: 1px white dashed;
    border-radius: 1em;
  }

  .grid-item {
    padding: 10px;
    border-radius: 1em;
  }

  .title {
    grid-row: 1;
    grid-column-start: 1;
    grid-column-end: 6;
    background-color: rgba(255, 255, 255, 0.02);
  }

  .sub-title {
    grid-column-start: 1;
    grid-column-end: 3;
  }

  .sub-values {
    grid-column-start: 3;
    grid-column-end: 6;
  }

  .slider {
    grid-row: 2;
  }

  .projects {
    grid-row: 3;
  }

  .actions {
    grid-row: 4;
  }

  ul.activity-filter li {
    display: inline;
    list-style-type: none;
    padding-right: 20px;
  }
</style>
