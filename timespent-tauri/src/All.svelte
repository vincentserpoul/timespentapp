<script lang="ts">
  import type { YActivities } from "../../timespent/bindings/YActivities";

  export let projects: [string, number][] = [];
  export let actions: [string, number][] = [];
</script>

<div>
  <h2>all</h2>
  <h3>projects</h3>
  <ul>
    {#each projects as project}
      <li>{project[0]}: {project[1]}%</li>
    {/each}
  </ul>
  <h3>actions</h3>
  <ul>
    {#each actions as action}
      <li>{action[0]}: {action[1]}%</li>
    {/each}
  </ul>
</div>
