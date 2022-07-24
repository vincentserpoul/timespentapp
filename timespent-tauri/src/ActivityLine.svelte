<script lang="ts">
  import type { Activity } from "../../timespent/bindings/Activity";
  import Project from "./Project.svelte";
  import Action from "./Action.svelte";

  export let activity: Activity;
</script>

<div>
  {activity.start_datetime}-{activity.end_datetime}:
  {#each activity.projects as project}
    <Project {project} />
  {/each}
  {#each activity.actions as action}
    <Action {action} />
  {/each}
  {activity.description}
</div>
