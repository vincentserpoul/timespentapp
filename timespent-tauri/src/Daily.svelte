<script lang="ts">
  import type { DailyAggregate } from "../../timespent/bindings/DailyAggregate";
  import Lines from "./Lines.svelte";
  export let daily: Array<DailyAggregate> = [];
</script>

<div>
  <h2>daily</h2>
  {#each daily as d}
    <p><b>day: {d.day}</b></p>
    <p>duration: {d.total_minutes}</p>
    <h3>projects</h3>
    <ul>
      {#each Object.entries(d.project_minutes) as p}
        <li>{p[0]}: {p[1]}</li>
      {/each}
    </ul>
    <h3>actions</h3>
    <ul>
      {#each Object.entries(d.action_minutes) as a}
        <li>{a[0]}: {a[1]}</li>
      {/each}
    </ul>
  {/each}
  <h2>chart</h2>
  <Lines stats={daily} />
</div>
