use chrono::{NaiveDate, NaiveDateTime};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Action {
    Review,
    Meeting,
    Research,
    Code,
    Docs,
    Unknown,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "review" => Ok(Action::Review),
            "meeting" => Ok(Action::Meeting),
            "research" => Ok(Action::Research),
            "code" => Ok(Action::Code),
            "docs" => Ok(Action::Docs),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, TS, Hash, Clone)]
#[ts(export)]
pub enum Type {
    Action(Action),
    Project(String),
}

impl FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match Action::from_str(input) {
            Ok(a) => Ok(Type::Action(a)),
            _ => Ok(Type::Project(input.to_string())),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Activity {
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub description: String,
    pub action: Action,
    pub projects: HashSet<String>,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, TS)]
pub struct Activities(pub Vec<Activity>);

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ActivitiesAggregate(
    pub NaiveDate,
    pub NaiveDate,
    pub HashSet<Action>,
    pub HashSet<String>,
);

impl Activities {
    pub fn aggregate_all(&self) -> ActivitiesAggregate {
        // find the first date, last date and activities range dates
        self.0.iter().fold(
            ActivitiesAggregate(
                NaiveDate::from_ymd(9999, 1, 1),
                NaiveDate::from_ymd(0, 1, 1),
                HashSet::new(),
                HashSet::new(),
            ),
            |mut act_agg: ActivitiesAggregate, activity| {
                act_agg.2.insert(activity.action);
                let projects = act_agg
                    .3
                    .union(&activity.projects)
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>();

                ActivitiesAggregate(
                    activity.start_datetime.date().min(act_agg.0),
                    activity.end_datetime.date().max(act_agg.1),
                    act_agg.2,
                    projects,
                )
            },
        )
    }

    pub fn filter(
        &self,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        actions: &HashSet<Action>,
        projects: &HashSet<String>,
        search_text: &Option<String>,
    ) -> Activities {
        self.0
            .clone()
            .into_iter()
            .filter(|activity| {
                activity.start_datetime.date() >= *start_date
                    && activity.end_datetime.date() <= *end_date
                    && actions.contains(&activity.action)
                    && activity.projects.iter().any(|proj| projects.contains(proj))
            })
            .filter(|activity| {
                if let Some(search) = search_text {
                    return activity.description.contains(search);
                }

                true
            })
            .collect::<Activities>()
    }
}

impl From<Vec<Activity>> for Activities {
    fn from(activities: Vec<Activity>) -> Self {
        Activities(activities)
    }
}

impl FromIterator<Activity> for Activities {
    fn from_iter<I: IntoIterator<Item = Activity>>(iter: I) -> Self {
        let mut c = Activities(Vec::new());

        for i in iter {
            c.0.push(i);
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Action, Type};

    #[test]
    fn test_action_from_str() {
        assert_eq!(Action::from_str("review").unwrap(), Action::Review);
        assert_eq!(Action::from_str("meeting").unwrap(), Action::Meeting);
        assert_eq!(Action::from_str("research").unwrap(), Action::Research);
        assert_eq!(Action::from_str("code").unwrap(), Action::Code);
        assert_eq!(Action::from_str("docs").unwrap(), Action::Docs);
        assert!(Action::from_str("other").is_err());
    }

    #[test]
    fn test_type_from_str() {
        assert_eq!(
            Type::from_str("review").unwrap(),
            Type::Action(Action::Review)
        );
        assert_eq!(
            Type::from_str("meeting").unwrap(),
            Type::Action(Action::Meeting)
        );
        assert_eq!(
            Type::from_str("research").unwrap(),
            Type::Action(Action::Research)
        );
        assert_eq!(Type::from_str("code").unwrap(), Type::Action(Action::Code));
        assert_eq!(Type::from_str("docs").unwrap(), Type::Action(Action::Docs));
        assert_eq!(
            Type::from_str("other").unwrap(),
            Type::Project("other".to_string())
        );
    }

    #[test]
    fn test_activities_aggregate() {
        let activities = Activities(vec![
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 22).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Code,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },
            Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },
        ]);

        assert_eq!(
            activities.aggregate_all(),
            ActivitiesAggregate(
                NaiveDate::from_ymd(2022, 7, 22),
                NaiveDate::from_ymd(2022, 7, 25),
                [Action::Review, Action::Code].into(),
                ["tag2".to_string(), "tag3".to_string()].into(),
            )
        );
    }

    #[test]
    fn test_activities_filter() {
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

        assert_eq!(
            activities.filter(
                &NaiveDate::from_ymd(2022, 7, 23),
                &NaiveDate::from_ymd(2022, 7, 30),
                &[Action::Code, Action::Review].into(),
                &["tag2".to_string()].into(),
                &None,
            ),
            Activities(vec![Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },])
        );

        assert_eq!(
            activities.filter(
                &NaiveDate::from_ymd(2022, 7, 23),
                &NaiveDate::from_ymd(2022, 7, 30),
                &[Action::Code, Action::Review].into(),
                &["tag2".to_string(), "tag3".to_string()].into(),
                &Some("desc".to_string()),
            ),
            Activities(vec![Activity {
                start_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(12, 0, 0),
                end_datetime: NaiveDate::from_ymd(2022, 7, 25).and_hms(13, 0, 0),
                description: "description".to_string(),
                action: Action::Review,
                projects: ["tag2".to_string(), "tag3".to_string()].into(),
            },])
        );

        assert_eq!(
            activities.filter(
                &NaiveDate::from_ymd(2022, 7, 23),
                &NaiveDate::from_ymd(2022, 7, 30),
                &[Action::Code, Action::Review].into(),
                &["tag2".to_string(), "tag3".to_string()].into(),
                &Some("proto".to_string()),
            ),
            Activities(vec![])
        );
    }
}
