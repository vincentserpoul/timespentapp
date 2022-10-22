<script lang="ts">
  import type { Filter } from "../../timespent/bindings/Filter";
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import { selected_scale } from "./stores";

  export let all_x_labels: ScaleXSegments;
  export let all_filter: Filter;

  export let applyFilter: () => Promise<void>;

  import { filter } from "./stores";

  import ScalesComponent from "./Scales.svelte";
  import ActivityButton from "./ActivityButton.svelte";
  import Slider from "./Slider.svelte";

  import { displayedXLabels } from "./display";
</script>

<div class="grid-container">
  <div class="grid-item title">
    <h1>filter</h1>
  </div>
  {#if all_x_labels}
    <div class="grid-item sub-title scales">
      <h2>scale</h2>
    </div>
    <div class="grid-item sub-values scales">
      <ScalesComponent />
    </div>
    <div class="grid-item sub-title slider">
      <h2>time range</h2>
    </div>
    <div class="grid-item sub-values slider">
      {#if all_x_labels[$selected_scale].length >= 2}
        <Slider
          labels={displayedXLabels(all_x_labels[$selected_scale])}
          {applyFilter}
        />
      {/if}
    </div>
    <div class="grid-item sub-title projects"><h2>projects</h2></div>
    <div class="grid-item sub-values projects">
      <ul class="activity-filter">
        {#each all_filter.projects as project}
          <li>
            <ActivityButton
              activity={project}
              selected={$filter.projects.includes(project)}
              click={() => {
                $filter.projects = $filter.projects.includes(project)
                  ? $filter.projects.filter((p) => p !== project)
                  : [...$filter.projects, project];
                applyFilter();
              }}
            />
          </li>
        {/each}
        <li>
          <a
            class="activity-reset"
            on:click={() => {
              $filter.projects = [];
              applyFilter();
            }}>deselect all</a
          >
        </li>
      </ul>
    </div>
    <div class="grid-item sub-title actions"><h2>actions</h2></div>
    <div class="grid-item sub-values actions">
      <ul class="activity-filter">
        {#each all_filter.actions as action}
          <li>
            <ActivityButton
              activity={action}
              selected={$filter.actions.includes(action)}
              click={() => {
                $filter.actions = $filter.actions.includes(action)
                  ? $filter.actions.filter((a) => a !== action)
                  : [...$filter.actions, action];
                applyFilter();
              }}
            />
          </li>
        {/each}
        <li>
          <a
            class="activity-reset"
            on:click={() => {
              $filter.actions = [];
              applyFilter();
            }}>deselect all</a
          >
        </li>
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

  .scales {
    grid-row: 2;
  }

  .slider {
    grid-row: 3;
  }

  .projects {
    grid-row: 4;
  }

  .actions {
    grid-row: 5;
  }

  ul.activity-filter li {
    display: inline;
    list-style-type: none;
    padding-right: 20px;
  }

  .activity-reset {
    border: 1px solid white;
    border-radius: 1em;
    padding: 0.5em;
    font-size: x-large;
    text-transform: lowercase;
    margin: 8px 16px;
    transition: 0.4s;
    cursor: pointer;
    color: white;
  }
</style>
