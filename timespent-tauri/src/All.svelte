<script lang="ts">
  import type { AllAggregate } from "../../timespent/bindings/AllAggregate";

  export let all: AllAggregate;
</script>

<div>
  <h2>all</h2>
  total hours: {all.total_minutes}
  <h3>projects</h3>
  <ul>
    {#each Object.entries(all.project_minutes) as p}
      <li>{p[0]}: {p[1]}</li>
    {/each}
  </ul>
  <h3>actions</h3>
  <ul>
    {#each Object.entries(all.action_minutes) as a}
      <li>{a[0]}: {a[1]}</li>
    {/each}
  </ul>
</div>
