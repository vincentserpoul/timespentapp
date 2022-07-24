<script lang="ts">
  import type { Activity } from "../../timespent/bindings/Activity";

  let items: Activity[];

  // With the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  invoke("load_directory")
    .then((response) => {
      items = response as Activity[];
    })
    .catch((error) => {
      console.error(error);
    });

  import ActivityLine from "./ActivityLine.svelte";
</script>

<ul>
  {#if items}
    {#each items as activity}
      <li><ActivityLine {activity} /></li>
    {/each}
  {:else}
    <li>Empty list</li>
  {/if}
</ul>

<style>
</style>
