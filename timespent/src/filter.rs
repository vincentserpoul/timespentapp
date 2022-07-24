use crate::activity::{Action, Activities, Activity};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Filter {
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
    pub free_text: String,
    pub projects: HashSet<String>,
    pub actions: HashSet<Action>,
}

impl Filter {
    pub fn new(activities: Activities) -> Self {
        let (min_datetime, max_datetime, projects, actions) = activities.0.iter().fold(
            (
                activities.0[0].start_datetime,
                activities.0[0].start_datetime,
                HashSet::new(),
                HashSet::new(),
            ),
            |(min, max, projects, actions), activity| {
                (
                    min.min(activity.start_datetime),
                    max.max(activity.end_datetime),
                    projects.union(&activity.projects).cloned().collect(),
                    actions.union(&activity.actions).cloned().collect(),
                )
            },
        );

        Filter {
            min_date: min_datetime.date(),
            max_date: max_datetime.date(),
            free_text: "".to_string(),
            projects,
            actions,
        }
    }
}

pub trait Filterable {
    fn filter(&self, filter: &Filter) -> Self;
}

impl Filterable for Activities {
    fn filter(&self, filter: &Filter) -> Self {
        self.0
            .iter()
            .filter(|a| {
                a.start_datetime.date() >= filter.min_date
                    && a.end_datetime.date() <= filter.max_date
                    && a.description.contains(&filter.free_text)
                    && a.projects.iter().all(|p| filter.projects.contains(p))
                    && a.actions.iter().all(|a| filter.actions.contains(a))
            })
            .cloned()
            .collect::<Vec<Activity>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Action, Activity};
    use chrono::NaiveDateTime;

    #[test]
    fn test_new_filter() {
        let activities = vec![
            Activity {
                start_datetime: NaiveDateTime::parse_from_str("2022-02-05 12:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                end_datetime: NaiveDateTime::parse_from_str("2022-02-05 13:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                description: "description".to_string(),
                projects: vec!["tag2".to_string(), "tag3".to_string()]
                    .into_iter()
                    .collect(),
                actions: vec![Action::Code].into_iter().collect(),
            },
            Activity {
                start_datetime: NaiveDateTime::parse_from_str("2022-02-06 12:00", "%Y-%m-%d %H:%M")
                    .unwrap(),
                end_datetime: NaiveDateTime::parse_from_str("2022-02-06 13:01", "%Y-%m-%d %H:%M")
                    .unwrap(),
                description: "description".to_string(),
                projects: vec!["tag2".to_string(), "tag3".to_string()]
                    .into_iter()
                    .collect(),
                actions: vec![Action::Review].into_iter().collect(),
            },
        ];

        let expected_filter = Filter {
            min_date: NaiveDate::parse_from_str("2022-02-05", "%Y-%m-%d").unwrap(),
            max_date: NaiveDate::parse_from_str("2022-02-06", "%Y-%m-%d").unwrap(),
            free_text: "".to_string(),
            projects: vec!["tag2".to_string(), "tag3".to_string()]
                .into_iter()
                .collect(),
            actions: vec![Action::Code, Action::Review].into_iter().collect(),
        };

        assert_eq!(Filter::new(activities.into()), expected_filter);
    }
}
