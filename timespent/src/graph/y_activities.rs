use super::scale::Scale;
use super::x_segments::ScaleXSegments;
use crate::activity::{Action, Activities};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use ts_rs::TS;

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct YActivities {
    pub scale_total_minutes: HashMap<Scale, Vec<i64>>,
    pub scale_actions_total_minutes: HashMap<Scale, HashMap<Action, Vec<i64>>>,
    pub scale_projects_total_minutes: HashMap<Scale, HashMap<String, Vec<i64>>>,
}

impl YActivities {
    pub fn new(
        activities: &Activities,
        actions: &HashSet<Action>,
        projects: &HashSet<String>,
        sxs: &ScaleXSegments,
    ) -> Self {
        // init each scale/xsegments with a vector of 0s
        let mut scale_total_minutes: HashMap<Scale, Vec<i64>> = sxs
            .values
            .iter()
            .map(|(scale, segments)| (*scale, vec![0i64; segments.len()]))
            .collect();

        let mut scale_actions_total_minutes: HashMap<Scale, HashMap<Action, Vec<i64>>> = sxs
            .values
            .iter()
            .map(|(scale, segments)| {
                (
                    *scale,
                    actions
                        .iter()
                        .map(|action| (*action, vec![0i64; segments.len()]))
                        .collect(),
                )
            })
            .collect();

        let mut scale_projects_total_minutes: HashMap<Scale, HashMap<String, Vec<i64>>> = sxs
            .values
            .iter()
            .map(|(scale, segments)| {
                (
                    *scale,
                    projects
                        .iter()
                        .map(|project| (project.clone(), vec![0i64; segments.len()]))
                        .collect(),
                )
            })
            .collect();

        // loop through activities to fill in the right scale/xsegment
        activities.0.iter().for_each(|activity| {
            // loop through all possible scales
            Scale::iterator().for_each(|scale| {
                // find the next valid xsegment idx for this scale
                let curr_idx =
                    sxs.find_correponding_x_segment_idx(&scale, &activity.start_datetime);

                let curr_activity_time = activity
                    .end_datetime
                    .signed_duration_since(activity.start_datetime)
                    .num_minutes();

                // add the time for this activity to the total minutes
                scale_total_minutes.get_mut(&scale).unwrap()[curr_idx] += curr_activity_time;

                // add the time for this activity to the action total minutes
                scale_actions_total_minutes
                    .get_mut(&scale)
                    .unwrap()
                    .get_mut(&activity.action)
                    .unwrap()[curr_idx] += curr_activity_time;

                // add the time for this activity to the project total minutes
                activity.projects.iter().for_each(|project| {
                    scale_projects_total_minutes
                        .get_mut(&scale)
                        .unwrap()
                        .get_mut(project)
                        .unwrap()[curr_idx] += curr_activity_time
                });
            });
        });

        YActivities {
            scale_total_minutes,
            scale_actions_total_minutes,
            scale_projects_total_minutes,
        }
    }

    // pub fn filter_by_date(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> YActivities {
    //     let mut x_segments: HashMap<Scale, XSegments> = HashMap::new();

    //     self.0.iter().for_each(|(scale, scale_x_segments)| {
    //         let mut filtered_x_segments = Vec::new();
    //         for xsegment in scale_x_segments {
    //             if xsegment.start_datetime.date() >= *start_date
    //                 && xsegment.end_datetime.date() <= *end_date
    //             {
    //                 filtered_x_segments.push(xsegment.clone());
    //             }
    //         }
    //         x_segments.insert(*scale, filtered_x_segments);
    //     });

    //     ScaleXSegments(x_segments)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Action, Activities, Activity};
    use chrono::NaiveDate;

    #[test]
    fn test_new_y_activities() {
        let activities = Activities(vec![
            Activity {
                start_datetime: NaiveDate::from_ymd_opt(2022, 7, 20)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 7, 20)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                description: "activity 1".to_string(),
                action: Action::Code,
                projects: ["tag1".to_string(), "tag2".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd_opt(2022, 7, 21)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 7, 21)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                description: "activity 1".to_string(),
                action: Action::Code,
                projects: ["tag1".to_string(), "tag2".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd_opt(2022, 7, 22)
                    .unwrap()
                    .and_hms_opt(11, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 7, 22)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                description: "activity 1".to_string(),
                action: Action::Code,
                projects: ["tag3".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd_opt(2022, 7, 22)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 7, 22)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                description: "activity 1".to_string(),
                action: Action::Code,
                projects: ["tag1".to_string(), "tag2".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd_opt(2022, 7, 25)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                end_datetime: NaiveDate::from_ymd_opt(2022, 7, 25)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                description: "activity 2".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },
        ]);

        let agg_activities = activities.aggregate_all();

        let sxs = ScaleXSegments::new(&agg_activities.0, &agg_activities.1);

        let y_activities =
            YActivities::new(&activities, &agg_activities.2, &agg_activities.3, &sxs);

        assert_eq!(
            y_activities.scale_total_minutes[&Scale::Day],
            vec![60, 60, 120, 0, 0, 60],
            "day total minutes: {:?}",
            sxs.values[&Scale::Day],
        );
        assert_eq!(
            y_activities.scale_total_minutes[&Scale::Week],
            vec![240, 60],
            "week total minutes"
        );
        assert_eq!(
            y_activities.scale_total_minutes[&Scale::Month],
            vec![300],
            "month total minutes"
        );
        assert_eq!(
            y_activities.scale_total_minutes[&Scale::Year],
            vec![300],
            "year total minutes"
        );
        assert_eq!(
            y_activities.scale_total_minutes[&Scale::All],
            vec![300],
            "all total minutes"
        );

        assert_eq!(
            y_activities.scale_actions_total_minutes[&Scale::Day][&Action::Code],
            vec![60, 60, 120, 0, 0, 0]
        );
        assert_eq!(
            y_activities.scale_actions_total_minutes[&Scale::Day][&Action::Review],
            vec![0, 0, 0, 0, 0, 60]
        );
        assert_eq!(
            y_activities.scale_projects_total_minutes[&Scale::Day]["tag1"],
            vec![60, 60, 60, 0, 0, 0]
        );
        assert_eq!(
            y_activities.scale_projects_total_minutes[&Scale::Day]["tag2"],
            vec![60, 60, 60, 0, 0, 60]
        );
        assert_eq!(
            y_activities.scale_projects_total_minutes[&Scale::Day]["tag3"],
            vec![0, 0, 60, 0, 0, 60]
        );
    }
}
