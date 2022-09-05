<script lang="ts">
  import type { ScaleXSegments } from "../../timespent/bindings/ScaleXSegments";
  import type { YActivities } from "../../timespent/bindings/YActivities";
  import type { Scale } from "../../timespent/bindings/Scale";

  export let selected_scale: Scale;
  export let total_minutes_all: Number;
  export let labels: string[];
  export let y_activities: YActivities;

  import Lines from "./Lines.svelte";
  import Camembert from "./Camembert.svelte";

  function calcActivityPercents(
    total_minutes: number[],
    activity_minutes: number[]
  ): number[] {
    return activity_minutes.map((activity_minute, i) => {
      if (activity_minute == 0) {
        return 0;
      }
      return Math.round((activity_minute * 100) / total_minutes[i]);
    });
  }
</script>

<div class="grid-container">
  <div class="grid-item title projects">
    <h1>projects</h1>
  </div>
  <div class="grid-item doughnut projects">
    <Camembert
      activity_percents={Object.entries(
        y_activities.scale_projects_total_minutes["All"]
      )
        .map((activity) => [
          activity[0],
          Math.round(
            (Number(activity[1][0]) * 100) / Number(total_minutes_all)
          ),
        ])
        .reduce(
          (acc, activity) => {
            acc.labels.push(activity[0]);
            acc.values.push(activity[1]);
            return acc;
          },
          { labels: [], values: [] }
        )}
    />
  </div>
  <div class="grid-item graph projects">
    <Lines
      {labels}
      activity_percents={Object.entries(
        y_activities.scale_projects_total_minutes[selected_scale]
      ).map(([name, durations]) => [
        name,
        calcActivityPercents(
          y_activities.scale_total_minutes[selected_scale],
          durations
        ),
      ])}
    />
  </div>
  <div class="grid-item title actions"><h1>activities</h1></div>
  <div class="grid-item doughnut actions">
    <Camembert
      activity_percents={Object.entries(
        y_activities.scale_actions_total_minutes["All"]
      )
        .map((activity) => [
          activity[0],
          Math.round(
            (Number(activity[1][0]) * 100) / Number(total_minutes_all)
          ),
        ])
        .reduce(
          (acc, activity) => {
            acc.labels.push(activity[0]);
            acc.values.push(activity[1]);
            return acc;
          },
          { labels: [], values: [] }
        )}
    />
  </div>
  <div class="grid-item graph actions">
    <Lines
      {labels}
      activity_percents={Object.entries(
        y_activities.scale_actions_total_minutes[selected_scale]
      ).map(([name, durations]) => [
        name,
        calcActivityPercents(
          y_activities.scale_total_minutes[selected_scale],
          durations
        ),
      ])}
    />
  </div>
</div>

<style>
  .grid-container {
    display: grid;
    gap: 2em;
  }

  .grid-item {
    padding: 10px;
    border-radius: 1em;
  }

  .title {
    grid-column-start: 1;
    grid-column-end: 24;
    text-align: center;
    background-color: rgba(255, 255, 255, 0.02);
  }

  .doughnut {
    grid-column-start: 1;
    grid-column-end: 3;
  }

  .graph {
    grid-column-start: 4;
    grid-column-end: 24;
    background-color: rgba(255, 255, 255, 0.05);
    padding: 2em;
  }

  .title.projects {
    grid-row: 1;
  }

  .doughnut.projects {
    grid-row: 2;
  }

  .graph.projects {
    grid-row: 2;
  }

  .title.actions {
    grid-row: 3;
  }

  .doughnut.actions {
    grid-row: 4;
  }

  .graph.actions {
    grid-row: 4;
  }
</style>
