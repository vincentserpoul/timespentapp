// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Action } from "./Action";
import type { Scale } from "./Scale";

export interface YActivities { scale_total_minutes: Record<Scale, Array<bigint>>, scale_actions_total_minutes: Record<Scale, Record<Action, Array<bigint>>>, scale_projects_total_minutes: Record<Scale, Record<string, Array<bigint>>>, }