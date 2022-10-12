import type { Filter } from "../../timespent/bindings/Filter";
import type { Action } from "../../timespent/bindings/Action";
import type { ActivitiesAggregate } from "../../timespent/bindings/ActivitiesAggregate";
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

export async function getFilter(): Promise<[ActivitiesAggregate, Filter]> {
  let res = await invoke("get_filter", {});

  return [res[0] as ActivitiesAggregate, res[1] as Filter];
}

export async function applyFilter(
  ftr: Filter
): Promise<[ActivitiesAggregate, Filter]> {
  console.log("applyFilter", ftr);
  let res = await invoke("apply_filter", {
    filter: ftr,
  });
  return [res[0] as ActivitiesAggregate, res[1] as Filter];
}
