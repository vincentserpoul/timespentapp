import type { Filter } from "../../timespent/bindings/Filter";
import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
import type { YActivities } from "../../timespent/bindings/YActivities";

import { invoke } from "@tauri-apps/api/tauri";

export async function getGraph(): Promise<
  [Number, ScaleXSegments, YActivities]
> {
  let [scale_x_segments, y_activities]: [ScaleXSegments, YActivities] =
    await invoke("get_graph", {});
  let total_minutes_all = Number(y_activities.scale_total_minutes["All"][0]);
  return [total_minutes_all, scale_x_segments, y_activities];
}

export async function getFilter(): Promise<[ScaleXSegments, Filter, Filter]> {
  let res = await invoke("get_filter", {});

  return [res[0] as ScaleXSegments, res[1] as Filter, res[2] as Filter];
}

export async function applyFilter(ftr: Filter): Promise<void> {
  await invoke("apply_filter", {
    filter: ftr,
  });

  return;
}
