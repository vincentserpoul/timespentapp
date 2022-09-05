<script lang="ts">
  export let labels: string[] = [];

  import RangeSlider from "svelte-range-slider-pips";
</script>

<RangeSlider
  values={[0, labels.length - 1]}
  formatter={(v) => labels[v]}
  handleFormatter={(v) => labels[v]}
  pips
  max={labels.length - 1}
/>
