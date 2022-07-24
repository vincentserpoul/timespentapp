<script lang="ts">
  import type { Aggregates } from "../../timespent/bindings/Aggregates";

  import Daily from "./Daily.svelte";
  import All from "./All.svelte";

  export let aggregates: Aggregates;
</script>

<div>
  <h1>aggregates</h1>
  <All all={aggregates.all} />
  <Daily daily={aggregates.daily} />
</div>
