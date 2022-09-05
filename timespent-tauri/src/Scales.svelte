<script lang="ts">
  import type { Scale } from "../../timespent/bindings/Scale";

  const scales: Scale[] = ["Day", "Week", "Month", "Year"];

  import { selected_scale } from "./stores";
  function select_scale(s: Scale) {
    selected_scale.set(s);
  }
</script>

<ul class="scale-list">
  {#each scales as scale}
    <li
      class={scale === $selected_scale ? "selected" : ""}
      href="#"
      on:click={() => {
        select_scale(scale);
      }}
    >
      {scale}
    </li>
  {/each}
</ul>

<style>
  ul.scale-list {
    list-style: none;
    list-style-type: none;
    text-align: center;
  }

  ul.scale-list li {
    display: inline-block;
    cursor: pointer;
    width: 10em;
    padding: 1em;
    text-transform: lowercase;
    margin: 0 1em 0 1em;
    border: 1px solid white;
    border-radius: 1em;
  }

  .selected {
    font-weight: bold;
    color: black;
    background-color: yellow;
  }
</style>
