import type { Filter } from "../../timespent/bindings/Filter";
import type { Aggregates } from "../../timespent/bindings/Aggregates";

let filter: Filter;
let aggregates: Aggregates;

// When using the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

invoke("get_curr_state", {})
  .then((res) => {
    filter = res[0] as Filter;
    aggregates = res[1] as Aggregates;
  })
  .catch((e) => console.error(e));

import App from "./App.svelte";

const app = new App({
  target: document.body,
  props: {
    filter,
    aggregates,
  },
});

export default app;
