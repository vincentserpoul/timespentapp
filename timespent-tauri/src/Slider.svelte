<script lang="ts">
  export let labels: string[] = [];
  export let applyFilter: () => Promise<void>;

  let values = [0, labels.length - 1];
  import RangeSlider from "svelte-range-slider-pips";

  import { filter } from "./stores";
  $: {
    filter.update((f) => {
      f.min_date = labels[values[0]];
      f.max_date = labels[values[1]];
      return f;
    });

    applyFilter();
  }
</script>

<div class="time-range">
  <RangeSlider
    bind:values
    formatter={(v) => labels[v]}
    handleFormatter={(v) => labels[v]}
    pips
    float
    all="label"
    max={labels.length - 1}
  />
</div>

<style>
  .time-range {
    width: 90%;
  }
</style>
