<script lang="ts">
  import stringToColor from "./stringToColor";

  export let activity: string;
  export let selected: boolean;
  export let click: ()=>void;

  const color = stringToColor(activity);
  $: opacity = selected ? "1" : "0.4";
</script>

<!-- svelte-ignore a11y-invalid-attribute -->
<a class="activity" href="#" style:color style:opacity on:click={click}>
  {activity}
</a>

<style>
  .activity {
    font-size: x-large;
    text-transform: lowercase;
    margin: 8px 16px;
    transition: 0.4s;
    cursor: pointer;
  }
</style>
