use super::x_segments::ScaleXSegments;
use super::y_activities::YActivities;
use crate::activity::{Action, Activities, ActivitiesAggregate};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Filter {
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
    pub actions: HashSet<Action>,
    pub projects: HashSet<String>,
    pub description: Option<String>,
}

// Aggregates
// for each scale, we need the x-axis and a y-axis for each of the type of activities
#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Graph {
    pub all_activities: Activities,
    pub activities_aggregate: ActivitiesAggregate,
    pub applied_filter: Filter,
    pub filtered_per_scale_x_segments: ScaleXSegments,
    pub filtered_per_scale_y_activities: YActivities,
}

impl Graph {
    pub fn new(all_activities: &Activities) -> Graph {
        let activities_aggregate = all_activities.aggregate_all();
        // let ActivitiesAggregate(start_date, end_date, actions, projects) = activities_aggregate;

        let applied_filter = Filter {
            min_date: activities_aggregate.0,
            max_date: activities_aggregate.1,
            actions: activities_aggregate.2.clone(),
            projects: activities_aggregate.3.clone(),
            description: None,
        };

        let filtered_per_scale_x_segments =
            ScaleXSegments::new(&activities_aggregate.0, &activities_aggregate.1);
        let filtered_per_scale_y_activities = YActivities::new(
            all_activities,
            &activities_aggregate.2,
            &activities_aggregate.3,
            &filtered_per_scale_x_segments,
        );

        Graph {
            all_activities: all_activities.clone(),
            activities_aggregate,
            applied_filter,
            filtered_per_scale_x_segments,
            filtered_per_scale_y_activities,
        }
    }

    pub fn apply_filter(&mut self, filter: &Filter) {
        let filtered_activities = self.all_activities.filter(
            &filter.min_date,
            &filter.max_date,
            &filter.actions,
            &filter.projects,
            &filter.description,
        );

        self.activities_aggregate = filtered_activities.aggregate_all();

        self.applied_filter = filter.clone();
        self.filtered_per_scale_x_segments =
            ScaleXSegments::new(&filter.min_date, &filter.max_date);
        self.filtered_per_scale_y_activities = YActivities::new(
            &filtered_activities,
            &self.activities_aggregate.2,
            &self.activities_aggregate.3,
            &self.filtered_per_scale_x_segments,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::Activity;

    #[test]
    fn test_new_graph() {
        let activities = Activities(vec![
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Code,
                projects: ["tag2".to_string(), "tag1".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },
        ]);

        let graph = Graph::new(&activities);
        let act_agg = ActivitiesAggregate(
            NaiveDate::from_ymd(2022, 7, 22),
            NaiveDate::from_ymd(2022, 7, 25),
            [Action::Code, Action::Review].into(),
            ["tag1".to_string(), "tag2".to_string(), "tag3".to_string()].into(),
        );

        let filter = Filter {
            min_date: NaiveDate::from_ymd(2022, 7, 22),
            max_date: NaiveDate::from_ymd(2022, 7, 25),
            actions: [Action::Code, Action::Review].into(),
            projects: ["tag1".to_string(), "tag2".to_string(), "tag3".to_string()].into(),
            description: None,
        };

        let sxs = ScaleXSegments::new(
            &NaiveDate::from_ymd(2022, 7, 22),
            &NaiveDate::from_ymd(2022, 7, 25),
        );

        let y_act = YActivities::new(&activities, &act_agg.2, &act_agg.3, &sxs);

        assert_eq!(graph.all_activities, activities, "all activities");
        assert_eq!(graph.activities_aggregate, act_agg, "activity agg");
        assert_eq!(graph.applied_filter, filter, "filter");
        assert_eq!(graph.filtered_per_scale_x_segments, sxs, "x segments");
        assert_eq!(graph.filtered_per_scale_y_activities, y_act, "y activities");
    }

    #[test]
    fn test_graph_filter() {
        let activities = Activities(vec![
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(13, 0, 0),
                description: "act 1".to_string(),
                action: Action::Code,
                projects: ["tag1".to_string(), "tag2".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
                description: "act".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },
        ]);

        let mut graph = Graph::new(&activities);

        let filter = Filter {
            min_date: NaiveDate::from_ymd(2022, 7, 22),
            max_date: NaiveDate::from_ymd(2022, 7, 22),
            actions: [Action::Code, Action::Review].into(),
            projects: ["tag2".to_string()].into(),
            description: None,
        };
        graph.apply_filter(&filter);

        let sxs = ScaleXSegments::new(
            &NaiveDate::from_ymd(2022, 7, 22),
            &NaiveDate::from_ymd(2022, 7, 22),
        );
        let filtered_activities = activities.filter(
            &NaiveDate::from_ymd(2022, 7, 22),
            &NaiveDate::from_ymd(2022, 7, 22),
            &[Action::Code].into(),
            &["tag2".to_string()].into(),
            &None,
        );
        let act_agg = filtered_activities.aggregate_all();

        let y_act = YActivities::new(&filtered_activities, &act_agg.2, &act_agg.3, &sxs);

        assert_eq!(graph.all_activities, activities, "all activities");
        assert_eq!(graph.activities_aggregate, act_agg, "activity agg");
        assert_eq!(graph.applied_filter, filter, "filter");
        assert_eq!(graph.filtered_per_scale_x_segments, sxs, "x segments");
        assert_eq!(graph.filtered_per_scale_y_activities, y_act, "y activities");
    }
}
