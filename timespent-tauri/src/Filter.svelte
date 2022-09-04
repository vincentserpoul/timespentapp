<script lang="ts">
  import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
  import type { Filter } from "../../timespent/bindings/Filter";

  // When using the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  async function getFilter(): Promise<[ActivitiesAggregate, Filter]> {
    let res = await invoke("get_filter", {});

    return [res[0] as ActivitiesAggregate, res[1] as Filter];
  }

  import ActivityButton from "./ActivityButton.svelte";
</script>

{#await getFilter() then [activitiesAggregate, filter]}
  <div>
    <h1>filter</h1>
    {activitiesAggregate[0]}-{activitiesAggregate[1]}:
    {#if filter}
      <ul class="activity-filter">
        {#each activitiesAggregate[3] as project}
          <li>
            <ActivityButton activity={project} />
            {#if filter.projects.includes(project)}
              *
            {/if}
          </li>
        {/each}
      </ul>
      {#if filter}
        <ul class="activity-filter">
          {#each activitiesAggregate[2] as action}
            <li>
              <ActivityButton activity={action} />
              {#if filter.actions.includes(action)}
                *
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    {/if}
  </div>
{/await}

<style>
  ul.activity-filter li {
    display: inline;
    list-style-type: none;
    padding-right: 20px;
  }
</style>
